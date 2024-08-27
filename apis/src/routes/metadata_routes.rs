use crate::{models::{metadata, user::AuthorizationHeaderToken, AppState}, handlers::{get_metadata, set_metadata}};
use metadata::Metatrack;
use rocket_contrib::json::Json;



#[get("/metadata/<track_id>")]
pub fn metadata_get(
    token: AuthorizationHeaderToken,
    track_id: i32,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<String, Box<dyn  std::error::Error>> {
    let user_id = state.get_user_id_by_token(&token.id)?;
    let user_id = match user_id {
        Some(id) => id,
        None => return Ok(String::from("No user found")),
    };


    let json = get_metadata(user_id, track_id, &state)?;
    Ok(json)
}

#[post("/metadata/<track_id>", format = "json", data = "<metadata>")]
pub fn metadata_post(
    token: AuthorizationHeaderToken,
    track_id: String,
    metadata: Json<Metatrack>,
    state: rocket::State<'_, AppState>,
) -> std::result::Result<Option<String>, Box<dyn  std::error::Error>> {
    let user_token = token.id.clone();
    let user_id = state.get_user_id_by_token(&user_token)?;
    let track_metadata: Metatrack = Metatrack {
        title: metadata.title.clone(),
        artist: metadata.artist.clone(),
        genre: metadata.genre.clone(),
        artstyle: metadata.artstyle.clone(),
        bpm: metadata.bpm.clone(),
        offset: metadata.offset.clone(),
        scale: metadata.scale.clone(),
        lyrics: metadata.lyrics.clone(),
    };

    match user_id {
        Some(user_id) => {
            let track_id = track_id.parse::<i32>()?;
            if track_id == -1 || user_id == -1 {
                return Ok(Some("No user or track found".to_string()));
            }

            let result = set_metadata(track_id, user_id,&track_metadata, &state)?;
            Ok(result)
        }
        None => Ok(None),
    }
}

