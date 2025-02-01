use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct RemoveInterestCohort;

#[rocket::async_trait]
impl Fairing for RemoveInterestCohort {
    fn info(&self) -> Info {
        Info {
            name: "Remove interest-cohort from Permissions-Policy header",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some(header) = response.headers().get_one("Permissions-Policy") {
         
            let filtered: Vec<&str> = header
                .split(',')
                .map(|directive| directive.trim())
                .filter(|directive| !directive.starts_with("interest-cohort"))
                .collect();

            if !filtered.is_empty() {
                let new_header = filtered.join(", ");
                response.set_raw_header("Permissions-Policy", new_header);
            } else {
                response.remove_header("Permissions-Policy");
            }
        }
    }
}
