#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod handlers;
mod models;
mod utils;
mod routes; // Import the routes module

use routes::make_cors;

use crate::models::AppState;

// OPTIONS requests are used by browsers to check if the server allows cross-origin requests
#[options("/<path..>")]
fn options_cors_preflight(path: std::path::PathBuf) -> std::io::Result<rocket::http::Status> {
    println!("CORS preflight request for path: {:?}", path);
    Ok(rocket::http::Status::Ok)
}

fn main() {
    dotenv::dotenv().ok();

    let cors = make_cors();
    // let db_pool = establish_connection();
    let app_state = AppState::new();

    rocket::ignite()
        .manage(app_state)
        .attach(cors.to_cors().unwrap())
        .mount(
            "/api",
               routes![
                    options_cors_preflight,
                                       
                   routes::waveform,
                   
                   routes::scenes, 
                   routes::correlation,    
                   
                   
                   routes::upload,
                   routes::file,

                   routes::metadata_get, 
                   routes::metadata_post, 

                   routes::get_state,

                   routes::features,
               ],
        )
        .mount(
            "/auth",
            routes![
                routes::user,
            ],
        )
        .launch();
}
