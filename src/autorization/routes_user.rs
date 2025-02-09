use rocket::request::Outcome;
use rocket::Request;
use rocket::{form::Form, request::FromRequest};
use rocket::http::Status;
use crate::database::{insert_table::UserEx, sorting::DbConn};
use super::request_guard::guard_user::{AdminGuard, UserGuard};
use super::token::check_token;
use super::{log::chek_pass, token::{create_for_user, GetUserOutcome, TokenCreating}, user::{user_creating, RegistrationOutcome}};


#[derive(FromForm)]
struct RegistrationForm {
    username: String,
    password: String,
}

#[post("/register", data = "<form>")]
pub async fn register(db: DbConn, cookies: &CookieJar<'_>, form: Form<RegistrationForm>) -> Result<Redirect, Status> {
    match user_creating(&db, &form.username, &form.password).await {
        RegistrationOutcome::Ok(user) => {
            match create_for_user(&db, &user).await {
                TokenCreating::Ok(token) => {
                    cookies.add_private(("auth_token", token));
                    Ok(Redirect::to(uri!(profile)))
                }
                TokenCreating::Err => Err(Status::InternalServerError),
            }
        },
        RegistrationOutcome::AlreadyInUse => Err(Status::Conflict),
        RegistrationOutcome::WeakPassword => Err(Status::BadRequest),
        RegistrationOutcome::Other => Err(Status::InternalServerError),
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

use rocket::http::{Cookie, CookieJar};

#[post("/login", data = "<form>")]
pub async fn login(db: DbConn, cookies: &CookieJar<'_>, form: Form<LoginForm>) -> Result<Redirect, Status> {
    match chek_pass(&db, form.username.clone(), form.password.clone()).await {
        GetUserOutcome::Some(user) => {
            cookies.add_private(Cookie::new("user_role", user.role.clone()));
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            match create_for_user(&db, &user).await {
                TokenCreating::Ok(token) => {
                    cookies.add_private(("auth_token", token));
                    Ok(Redirect::to(uri!(profile)))
                }
                TokenCreating::Err => Err(Status::InternalServerError),
            }
        }
        GetUserOutcome::None => Err(Status::Unauthorized),
        GetUserOutcome::Error => Err(Status::InternalServerError),
    }
}



#[get("/profile")]
pub async fn profile(user: UserGuard) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.id);

    let template_name = match user.role.as_str() {
        "admin" => "admin_profile",
        "user" => "user_profile",
        _ => "access_denied",
    };

    Template::render(template_name, &context)
}

use rocket::{fairing::{Fairing, Info, Kind},Response};

pub struct NoCacheFairing;

#[rocket::async_trait]
impl Fairing for NoCacheFairing {
    fn info(&self) -> Info {
        Info {
            name: "NoCacheFairing",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {

        if let Outcome::Success(_) = request.guard::<AuthenticatedUser>().await {
            response.set_raw_header("Cache-Control", "no-store, no-cache, must-revalidate, private");
            response.set_raw_header("Pragma", "no-cache");
            response.set_raw_header("Expires", "0");
        }
    }
}


pub struct AuthenticatedUser {
    pub user: UserEx,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        if let Some(cookie) = cookies.get_private("auth_token") {
            let token_str = cookie.value();
            let db = match request.guard::<DbConn>().await {
                Outcome::Success(conn) => conn,
                _ => return Outcome::Error((Status::InternalServerError, ())),
            };

            match check_token(&db, token_str.to_string()).await 
            {
                GetUserOutcome::Some(user)=>Outcome::Success(AuthenticatedUser{user}),
                GetUserOutcome::Error=> Outcome::Error((Status::Unauthorized,())),
                GetUserOutcome::None => Outcome::Error((Status::ServiceUnavailable,())),
        }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("auth_token");
    Redirect::to("/")
}


#[get("/admin")]
pub async fn admin_panel(_admin: AdminGuard) -> Template {
    Template::render("admin_panel", &())
}
