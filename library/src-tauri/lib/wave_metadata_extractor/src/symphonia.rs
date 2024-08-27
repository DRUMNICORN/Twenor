// use symphonia::core::{
//     errors::Error,
//     formats::FormatOptions,
//     io::MediaSourceStream,
//     meta::{MetadataOptions, MetadataRevision},
//     probe::Hint,
// };

// use crate::log::Filename;
// use crate::log::Log;
// static LOG: Log = Log::new(Filename::TrackMetadataSymphonia);

// use std::collections::HashMap;

// fn get_extension(track_path: &str) -> String {
//     let track_path = track_path.to_lowercase();
//     let track_extension = track_path.split(".").last().unwrap();
//     track_extension.to_string()
// }

// pub fn load_revision(track_path: String) -> Result<MetadataRevision, Error> {
//     LOG.debug(&format!("Loading revision for {}", track_path));
//     let path: String = track_path.to_string();
//     let src = match std::fs::File::open(&path) {
//         Ok(file) => file,
//         Err(err) => {
//             LOG.error(&format!("Error opening file: {}", err));
//             return Err(Error::IoError(err));
//         }
//     };

//     let mss = MediaSourceStream::new(Box::new(src), Default::default());
//     let mut hint = Hint::new();
//     hint.with_extension(&get_extension(&track_path));
//     let meta_opts: MetadataOptions = Default::default();
//     let fmt_opts: FormatOptions = Default::default();
//     let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts);

//     match probed {
//         Err(e) => Err(e),
//         Ok(mut probed) => {
//             if let Some(metadata_rev) = probed.format.metadata().current() {
//                 Ok(metadata_rev.clone())
//             } else if let Some(metadata_rev) =
//                 probed.metadata.get().as_ref().and_then(|m| m.current())
//             {
//                 Ok(metadata_rev.clone())
//             } else {
//                 LOG.error(&format!("No metadata found for {}", track_path));
//                 Err(Error::Unsupported("No metadata found"))
//             }
//         }
//     }
// }

// pub fn get_metadata(track_path: &str) -> HashMap<String, String> {
//     LOG.debug(&format!("Getting metadata for {}", track_path));
//     let revision = load_revision(track_path.to_string());
//     if revision.is_err() {
//         return HashMap::new();
//     }
//     let metadata = extract_metadata(&revision.unwrap());
//     LOG.debug("Loaded metadata");
//     metadata
// }

// fn extract_metadata(metadata_rev: &MetadataRevision) -> HashMap<String, String> {
//     LOG.debug("Extracting metadata");
//     let mut metadata = HashMap::new();
//     // loop as long as valid values are found
//     for i in 0..metadata_rev.nodes().len() {
//         let node = metadata_rev.nodes().get(i);
//         if let Some(node) = node {
//             let mut value = node.value.to_string();
//             // split value if needed
//             if value.len() > 73 {
//                 value = value.split_at(73).0.to_owned();
//             }

//             let std_key = node.std_key;
//             if std_key.is_none() {
//                 let val = value.clone();

//                 let key = node.key.to_string().to_lowercase();
//                 metadata.insert(key, val);
//             }

//             if let Some(std_key) = std_key {
//                 let val = value.clone();
//                 let std_key = format!("{:?}", std_key).to_lowercase();
//                 metadata.insert(std_key, val);
//             }
//         }
//     }
//     LOG.debug("Extracted metadata");
//     metadata
// }
