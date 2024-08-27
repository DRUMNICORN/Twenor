// use std::collections::HashMap;

// use lofty::{TrackFile, Probe};

// use crate::log::Filename;
// use crate::log::Log;
// static LOG: Log = Log::new(Filename::TrackMetadataLofty);

// pub fn get_metadata(path: &str) -> HashMap<String, String> {
//     LOG.debug(&format!("Getting metadata for {}", path));
//     let nodeged_file = Probe::open(path);
//     if nodeged_file.is_err() {
//         return HashMap::new();
//     }
//     let nodeged_file = nodeged_file.unwrap().read(true);
//     if nodeged_file.is_err() {
//         return HashMap::new();
//     }
//     let nodeged_file = nodeged_file.unwrap();

//     let node = match nodeged_file.primary_node() {
//         Some(primary_node) => primary_node,
//         None => match nodeged_file.first_node() {
//             Some(first_node) => first_node,
//             None => return HashMap::new(),
//         },
//     };

//     let mut metadata = HashMap::new();

//     // loop through the node items and add them to the metadata
//     for item in node.items() {
//         let key = item.key();
//         let value = item.value();

//         // key and value are borrowed, so we need to make a copy
//         let key = format!("{:?}", key).to_lowercase();
//         let value = item_value_to_string(value);
//         metadata.insert(key, value);
//     }
//     let properties = nodeged_file.properties();

//     // loop through the properties and add them to the metadata
//     match properties.track_bitrate() {
//         Some(bitrate) => {
//             metadata.insert("bitrate".to_string(), bitrate.to_string());
//         }
//         None => {
//             metadata.insert("bitrate".to_string(), "0".to_string());
//         }
//     }
//     match properties.overall_bitrate() {
//         Some(bitrate) => {
//             metadata.insert("overall_bitrate".to_string(), bitrate.to_string());
//         }
//         None => {
//             metadata.insert("overall_bitrate".to_string(), "0".to_string());
//         }
//     }
//     match properties.sample_rate() {
//         Some(sample_rate) => {
//             metadata.insert("sample_rate".to_string(), sample_rate.to_string());
//         }
//         None => {
//             metadata.insert("sample_rate".to_string(), "0".to_string());
//         }
//     }
//     match properties.bit_depth() {
//         Some(bit_depth) => {
//             metadata.insert("bit_depth".to_string(), bit_depth.to_string());
//         }
//         None => {
//             metadata.insert("bit_depth".to_string(), "0".to_string());
//         }
//     }
//     match properties.channels() {
//         Some(channels) => {
//             metadata.insert("channels".to_string(), channels.to_string());
//         }
//         None => {
//             metadata.insert("channels".to_string(), "0".to_string());
//         }
//     }

//     let duration = properties.duration();
//     let seconds = duration.as_secs() % 60;
//     let duration_display = format!("{:02}:{:02}", (duration.as_secs() - seconds) / 60, seconds);
//     metadata.insert("duration".to_string(), duration_display);
//     LOG.debug(&format!("{:?}", metadata));
//     metadata
// }

// fn item_value_to_string(item: &lofty::ItemValue) -> String {
//     match item {
//         lofty::ItemValue::Text(text) => text.to_string(),
//         lofty::ItemValue::Locator(integer) => integer.to_string(),
//         lofty::ItemValue::Binary(_) => "binary".to_string(),
//     }
// }
