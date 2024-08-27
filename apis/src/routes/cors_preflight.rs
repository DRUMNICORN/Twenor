
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};



pub fn make_cors() -> CorsOptions {
    let allowed_origins = AllowedOrigins::default();

    
    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    cors
}
