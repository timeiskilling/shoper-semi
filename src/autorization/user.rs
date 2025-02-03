use crate::database::insert_table::NewUser;
use crate::database::sorting::DbConn;
use crate::schema::users::password_hash;
use argon2::password_hash::rand_core::{self, OsRng};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

pub enum RegistrationOutcome {
    Ok,
    AlreadyInUse,
    WeakPassword,
    Other,
}

async fn user_creating(db : DbConn, login : &str, password : &str)  {
    // if password.len() < 8 {
    //     return RegistrationOutcome::WeakPassword;
    // }

    let sault = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hashh = argon2.
        hash_password(password.as_bytes(), &sault).unwrap();

    let new_user = NewUser {
        username : login,
        password_hash: password_hashh.to_string().as_str(),
        role: "user"
    };
}