
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

pub fn make_cors() -> CorsOptions {
    let allowed_origins = AllowedOrigins::some_exact(&["https://drumni.com", "http://localhost:3000"]);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    cors
}
