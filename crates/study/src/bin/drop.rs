use std::fs::{self, File};
use std::io::Result;
use std::path::PathBuf;

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    // 1. Define the `new` associated function

    pub fn new(file: impl AsRef<str>) -> Result<Self> {
        File::create(file.as_ref())?;

        Ok(Self {
            path: PathBuf::from(file.as_ref()),
        })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!("Failed to remove temp file {:?}: {}", self.path, e);
        }
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
