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

pub async  fn create_for_user(db : &DbConn, user : &UserEx) -> TokenCreating {
    let mut token_bytes = [0u8, 32];
    rand_core::OsRng.fill_bytes(&mut token_bytes);
    let token_entry = general_purpose::STANDARD.encode(token_bytes);
    let user_ids = user.id.clone();

    let result = db.run(move |conn| {
        let token_str = general_purpose::STANDARD.encode(token_bytes); 
        let new_token = NewToken {
            token: &token_str,
            user_id: user_ids,
            token_type: "user_acces",
        };

        diesel::insert_into(tokens::table)
        .values(new_token)
        .execute(conn)
    }).await;

    match result {
        Ok(_) => TokenCreating::Ok(token_entry),
        Err(e) => { 
            eprintln!("Error inserting token {}",e);
            TokenCreating::Err
        }
    }

}


pub async fn check_token(db : DbConn , token : String) -> GetUserOutcome {
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