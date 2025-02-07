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
pub async fn register(db: DbConn, form: Form<RegistrationForm>) -> Status {
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

use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[post("/login", data = "<form>")]
pub async fn login(db: DbConn, form: Form<LoginForm>) -> Result<Template, Status> {
    match chek_pass(&db, form.username.clone(), form.password.clone()).await {
        GetUserOutcome::Some(user) => {
            // За бажанням, ви можете зберегти токен у сесії або печиві
            // match create_for_user(&db, &user).await {
            //     TokenCreating::Ok(token) => {
                    // Наприклад, зберегти токен або іншу інформацію
            //     }
            //     TokenCreating::Err => return Err(Status::InternalServerError),
            // }

            // Підготуємо дані для шаблону
            let mut context = HashMap::new();
            context.insert("username", user.username.clone());
            context.insert("role", user.role.clone());
            // Додайте інші необхідні поля з `user`

            // Повертаємо шаблон з контекстом
            Ok(Template::render("user_profile", &context))
        }
        GetUserOutcome::None => Err(Status::Unauthorized),
        GetUserOutcome::Error => Err(Status::InternalServerError),
    }
}

