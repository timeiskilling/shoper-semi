use argon2::password_hash::rand_core::{self, RngCore};
use base64::{engine::general_purpose, Engine as _};
use diesel::prelude::*;
use crate::{database::{insert_table::{NewToken, UserEx}, sorting::DbConn}, schema::{tokens, users}};

pub enum TokenCreating {
    Ok(String),
    Err,
}

pub enum GetUserOutcome {
    Some(UserEx),
    None,
    Error,
}



pub async fn create_for_user(db: &DbConn, user: &UserEx) -> TokenCreating {
    let user_id = user.id;

    db.run(move |conn| {
        diesel::delete(tokens::table.filter(tokens::user_id.eq(user_id)))
            .execute(conn)
            .expect("Error deleting old tokens");

        let mut token_bytes = [0u8; 32];
        rand_core::OsRng.fill_bytes(&mut token_bytes);
        let token_str = general_purpose::STANDARD.encode(token_bytes);

        let now = chrono::Utc::now().naive_utc();
        let expires = now + chrono::Duration::days(30);

        let new_token = NewToken {
            token: &token_str,
            user_id,
            token_type: "user_access",
            issued_at: now,
            expires_at: expires,
        };

        diesel::insert_into(tokens::table)
            .values(new_token)
            .execute(conn)
            .map(|_| TokenCreating::Ok(token_str))
            .unwrap_or(TokenCreating::Err)
    })
    .await
}


pub async fn check_token(db : &DbConn , token : String) -> GetUserOutcome {
    db.run(move |conn| {
        match users::table
            .left_join(tokens::table.on(tokens::user_id.eq(users::id)))
            .select((users::id,users::username,users::password_hash,users::role))
            .filter(tokens::token.eq(token))
            .get_result::<UserEx>(conn)
        
        {
            Ok(user) => GetUserOutcome::Some(user),
            Err(diesel::result::Error::NotFound) => GetUserOutcome::None,
            _ => GetUserOutcome::Error,
        }
    }).await
}