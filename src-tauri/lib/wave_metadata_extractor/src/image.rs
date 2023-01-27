// // #![warn(missing_docs)]

// //! A command-line tool to embed images into mp3 files. The real work is done by the "id3" crate,
// //! but this project makes it easier to deal with embedded cover art in particular.

// use std::path::Path;

// use anyhow::anyhow;
// // use id3::NodeLike;

// use id3::{Node};
// use image;

// /// Extract the first found embedded image from `music_filename` and write it as a file with the
// /// given `image_filename`. The image file will be silently overwritten if it exists.
// ///
// /// Any errors from parsing id3 nodes will be propagated. The function will also return an error if
// /// there's no embedded images in the mp3 file.
// ///
// pub fn extract_first_image(music_filename: &Path, image_filename: &Path) -> anyhow::Result<()> {
//     let node = read_node(music_filename)?;
//     let first_picture = node.pictures().next();

//     if let Some(p) = first_picture {
//         match image::load_from_memory(&p.data) {
//             Ok(image) => {
//                 image.save(&image_filename).
//                     map_err(|e| anyhow!("Couldn't write image file {:?}: {}", image_filename, e))?;
//             },
//             Err(e) => return Err(anyhow!("Couldn't load image: {}", e)),
//         };

//         Ok(())
//     } else {
//         Err(anyhow!("No image found in music file"))
//     }
// }

// fn read_node(path: &Path) -> anyhow::Result<Node> {
//     Node::read_from_path(&path).or_else(|e| {
//         e.partial_node.clone().ok_or_else(|| anyhow!("Error reading music file {:?}: {}", path, e))
//     })
// }

// pub fn request_local_image(track_path: &str, id: &str) -> String {
//   LOG.debug(&format!("Requesting local image {}", track_path));
//   let track_path = Path::new(&track_path);
//   let path = format!("../data/images/{}.jpg", id);
//   let image_path = Path::new(&path);
//   match read_image::extract_first_image(track_path, image_path) {
//       Ok(()) => {
//           LOG.debug(&format!("Local image for {} found", track_path.display()));
//           format!("./data/images/{}.jpg", id)
//       }
//       Err(_e) => {
//           LOG.warn(&format!("No image found for {}", track_path.display()));
//           "./data/img/default.png".to_string()
//       }
//   }
// }
