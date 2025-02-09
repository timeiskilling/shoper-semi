use crate::database::insert_table::{NewUser, UserEx};
use crate::database::sorting::DbConn;
use crate::schema::users::{self, password_hash};
use argon2::password_hash::rand_core::{self, OsRng};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::RunQueryDsl;
use crate::schema::users::dsl::*;

pub enum RegistrationOutcome {
    Ok(UserEx),
    AlreadyInUse,
    WeakPassword,
    Other,
}

pub async fn admin_creating(db : &DbConn,) -> RegistrationOutcome {
    let sault = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password = "admins_second";

    let password_hashh = argon2.
        hash_password(password.as_bytes(), &sault).unwrap();

        let new_user = NewUser {
            username : "admin".to_string(),
            password_hash: password_hashh.to_string(),
            role: "admin".to_string()
        };

        let result = db.run(move |conn| {
            diesel::insert_into(users)
                .values(&new_user)
                .get_result::<UserEx>(conn)
        }).await;
    
        match result {
            Ok(user) => RegistrationOutcome::Ok(user),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => RegistrationOutcome::AlreadyInUse,
            _ => RegistrationOutcome::Other,
        }
}


fn main()  {

}

pub async fn user_creating(db : &DbConn, login : &str, password : &str) -> RegistrationOutcome {
    if password.len() < 8 {
        return RegistrationOutcome::WeakPassword;
    }
    let sault = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hashh = argon2.
        hash_password(password.as_bytes(), &sault).unwrap();

    let new_user = NewUser {
        username : login.to_string(),
        password_hash: password_hashh.to_string(),
        role: "user".to_string()
    };
    
    let result = db.run(move |conn| {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<UserEx>(conn)
    }).await;

    match result {
        Ok(user) => RegistrationOutcome::Ok(user),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => RegistrationOutcome::AlreadyInUse,
        _ => RegistrationOutcome::Other,
    }
}