// use musicbrainz_rs::{
//     entity::{
//         // recording::Recording as MusicBrainzRecording, 
//         release::Release as MusicBrainzRelease,
//         CoverartResponse,
//     },
//     FetchCoverart, 
//     // Search,
// };
// use std::collections::HashMap;

// use crate::log::Filename;
// use crate::log::Log;
// static LOG: Log = Log::new(Filename::TrackMetadataMusicBrainz);

// pub fn get_metadata(
//     track_path: &str,
//     metadata_global: HashMap<String, String>,#
//     release_name: &str,
//     artist_name: &str,
// ) -> HashMap<String, String> {
//     LOG.debug(&format!("Getting metadata for {}", track_path));

//     let query = format!(
//         "query=artist:{}%20AND%20title:{}&limit=1",
//         artist_name, release_name
//     );

//     let mut search = MusicBrainzRecording::search(query);
//     LOG.debug(&format!("Searching for {:?}", search));

//     let result = search
//         .execute()
//         .expect("Failed to query release group")
//         .entities;

//     LOG.debug(&format!("Found {} results", result.len()));
//     let result_metadata = result.first().unwrap();
//     let mut metadata = HashMap::new();

//     metadata.insert("id".to_string(), result_metadata.id.to_string());

//     let release = result_metadata.releases.as_ref().unwrap().first().unwrap();
//     let url = get_coverart_url(&release);
//     metadata.insert("image".to_string(), url);

//     LOG.debug(&format!("Loaded metadata"));
//     metadata
// }

// pub fn get_coverart_url(release: &MusicBrainzRelease) -> String {
//     let coverart = release
//         .get_coverart()
//         .front()
//         .res_250()
//         .execute()
//         .expect("Unable to get coverart");

//     // LOG.debug(&format!("Found {:?} coverart", coverart));

//     match coverart {
//         CoverartResponse::Url(coverart_url) => {
//             // LOG.debug(&format!("Found coverart url: {:?}", coverart_url));
//             coverart_url.to_string()
//         }
//         _ => "".to_string(),
//     }
// }
