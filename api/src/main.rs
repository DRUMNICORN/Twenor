#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod app_state;
mod logging_utils;

use models::{self};

use crate::{routes::make_cors, app_state::AppState};
mod routes;

// // OPTIONS requests are used by browsers to check if the server allows cross-origin requests
#[options("/<path..>")]
fn options_cors_preflight(path: std::path::PathBuf) -> std::io::Result<rocket::http::Status> {
    log::info!("OPTIONS request for path: {:?}", path);
    Ok(rocket::http::Status::Ok)
}

fn main() {
    match logging_utils::init_logging() {
        Ok(_) => {}
        Err(_e) => {
            panic!("Failed to start server");
        }
    };

    dotenv::dotenv().ok();
    log::info!("Starting server");
    let cors = make_cors();
    let cors = match cors.to_cors() {
        Ok(cors) => cors,
        Err(_e) => {
            panic!("Failed to start server");
        }
    };
    // let db_pool = establish_connection();
    let app_state = AppState::new();

    rocket::ignite()
        .manage(app_state)
        .attach(cors)
        .mount(
            "/api",
               routes![
                options_cors_preflight,
                                       
                   routes::delete_audio,
                   routes::get_audio,
                   routes::get_audios,
                   
                   routes::get_scenes, 
                   routes::get_correlation,    
                   
                   routes::get_download,
                   routes::post_upload,
                   routes::post_file,
                   routes::get_file,

                   routes::get_metadata, 
                   routes::post_metadata, 

                   routes::get_state,

                   routes::get_features,
 
                   routes::post_terms_accept,
                   routes::get_terms,
               ],
        )
    //    .mount("/", routes![get_videos])
        .mount(
            "/auth",
            routes![
                routes::user,
            ],
        )
        // main page redirects to "https://drumni.com"
        .mount("/", routes![routes::get_index])
        .launch();
}
