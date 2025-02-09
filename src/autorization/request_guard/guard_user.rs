use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;

pub struct UserGuard {
    pub id: i32,
    pub role: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let role = cookies.get_private("user_role").map(|cookie| cookie.value().to_string());
        let id = cookies.get_private("user_id").and_then(|cookie| cookie.value().parse::<i32>().ok());

        match (id, role) {
            (Some(id), Some(role)) => Outcome::Success(UserGuard { id, role }),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}


pub struct AdminGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        if let Some(role) = cookies.get_private("user_role") {
            if role.value() == "admin" {
                return Outcome::Success(AdminGuard);
            }
        }
        Outcome::Error((Status::Forbidden, ()))
    }
}
