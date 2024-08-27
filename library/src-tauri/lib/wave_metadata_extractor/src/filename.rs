// use std::collections::HashMap;

// // use twenor_log::Filename;
// // use twenor_log::Log;
// // static LOG: Log = Log::new(Filename::TrackMetadataFilename);

// pub fn read(track_path: &str) -> HashMap<String, String> {
//     // LOG.debug(&format!("Getting metadata for {}", track_path));

//     let file_name = track_path
//         .split(".")
//         .next()
//         .unwrap()
//         .split("\\")
//         .last()
//         .unwrap();

//     let file_extension = track_path.split(".").last().unwrap();

//     let mut artist = "UnknownArtist".to_string();
//     let mut description = "".to_string();
//     let title: String;

//     if file_name.split("-").count() == 2 {
//         let title_artist = file_name.split("-").collect::<Vec<&str>>();
//         artist = title_artist[0].to_string();
//         title = title_artist[1].to_string();
//     } else if file_name.split("-").count() >= 3 {
//         let title_artist_description = file_name.split("-").collect::<Vec<&str>>();
//         artist = title_artist_description[0].to_string();
//         title = title_artist_description[1].to_string();
//         description = title_artist_description[2].to_string();
//     } else {
//       title = file_name.to_string();
//     }

//     let mut metadata = HashMap::new();
//     metadata.insert("trackartist".to_string(), artist);
//     metadata.insert("tracktitle".to_string(), title);
//     metadata.insert("description".to_string(), description);
//     metadata.insert("extension".to_string(), file_extension.to_string());

//     metadata
// }
