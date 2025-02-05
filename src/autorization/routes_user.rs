use rocket::form::Form;
use rocket::http::Status;
use crate::database::sorting::DbConn;
use super::{log::chek_pass, token::{create_for_user, GetUserOutcome, TokenCreating}, user::{user_creating, RegistrationOutcome}};

#[derive(FromForm)]
struct RegistrationForm {
    username: String,
    password: String,
}

#[post("/register", data = "<form>")]
async fn register(db: DbConn, form: Form<RegistrationForm>) -> Status {
    match user_creating(&db, &form.username, &form.password).await {
        RegistrationOutcome::Ok => Status::Ok,
        RegistrationOutcome::AlreadyInUse => Status::Conflict,
        RegistrationOutcome::WeakPassword => Status::BadRequest,
        RegistrationOutcome::Other => Status::InternalServerError,
    }
}



#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login", data = "<form>")]
async fn login(db: DbConn, form: Form<LoginForm>) -> Result<String, Status> {
    match chek_pass(&db, form.username.clone(), form.password.clone()).await {
        GetUserOutcome::Some(user) => {
            match create_for_user(&db, &user).await {
                TokenCreating::Ok(token) => Ok(token),
                TokenCreating::Err => Err(Status::InternalServerError),
            }
        }
        GetUserOutcome::None => Err(Status::Unauthorized),
        GetUserOutcome::Error => Err(Status::InternalServerError),
    }
}