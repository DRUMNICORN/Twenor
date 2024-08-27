extern crate rfd;

use std::path::Path;
use std::path::PathBuf;

extern crate anyhow;
use anyhow::anyhow as err;
use anyhow::Result;

use rfd::FileDialog;

pub fn input_file_dialog() -> Result<PathBuf> {
    let files = FileDialog::new()
        .add_filter("All", &["*"])
        .set_directory("/")
        .pick_file();

    match files {
        Some(file) => {
            let path = Path::new(&file);
            if path.exists() {
                Ok(path.to_path_buf())
            } else {
                Err(err!("File does not exist"))
            }
        }
        None => Err(err!("No file selected")),
    }
}
