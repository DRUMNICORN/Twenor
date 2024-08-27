use rocket::{request::{FromRequest, self}, Request, http::Status};
pub struct UserAuthorizationToken {
    id: String,
}

impl UserAuthorizationToken {
    pub fn get_id(&self) -> &String {
        &self.id
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAuthorizationToken {
    type Error = std::io::Error;

    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<UserAuthorizationToken, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return request::Outcome::Failure((
                Status::BadRequest,
                std::io::Error::new(std::io::ErrorKind::Other, "No authorization"),
            ));
        }
        let authorization_header = keys[0];
        let token = match authorization_header.strip_prefix("Bearer ") {
            Some(key) => key.to_string(),
            None => authorization_header.to_string(),
        };
        if token == "undefined" {
            log::info!("Canceling request: No authorization (missing token)");
            return request::Outcome::Failure((
                Status::BadRequest,
                std::io::Error::new(std::io::ErrorKind::Other, "No authorization"),
            ));
        }
        request::Outcome::Success(UserAuthorizationToken { id: token }) // TODO: Check if token is valid
    }
}
