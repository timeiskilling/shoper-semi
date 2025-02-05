use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::*;

use crate::{database::{insert_table::UserEx, sorting::DbConn}, schema::users};

use super::token::GetUserOutcome;



pub async fn chek_pass(db : &DbConn , login : String , password : String) -> GetUserOutcome {
    let result = db.run(move |conn|{
        users::table
            .filter(users::username.eq(login.to_lowercase()))
            .get_result::<UserEx>(conn)
    }).await;

    match result {
        Ok(user) => {
            let argon2 = Argon2::default();
            if let Ok(parsed_hash) = PasswordHash::new(&user.password_hash) {
                if argon2
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    GetUserOutcome::Some(user)
                } else {
                    GetUserOutcome::None
                }
            } else {
                GetUserOutcome::None
            }
        }
        Err(diesel::result::Error::NotFound) => GetUserOutcome::None,
        _ => GetUserOutcome::Error,
    }
}
