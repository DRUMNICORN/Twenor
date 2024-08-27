
        // main page redirects to "https://drumni.com"
        // .mount("/", routes![routes::index])

// Path: api/src/routes/index.routes.rs

use rocket::response::Redirect;

#[get("/")]
pub fn get_index() -> std::result::Result<Redirect, Box<dyn  std::error::Error>> {
    Ok(Redirect::to("https://drumni.com"))
}