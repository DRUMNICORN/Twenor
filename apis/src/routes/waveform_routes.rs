use crate::{models::user::AuthorizationHeaderToken, handlers::waveform_handler};

#[get("/waveform")]
pub fn waveform(token: AuthorizationHeaderToken) -> std::io::Result<String> {
    waveform_handler::get_waveform(token)
}