use std::fs::{File};
use std::path::Path;

pub struct FileHandler {
    pub input_file : File,
    pub output_file: File
}

impl FileHandler{
    pub fn new(input_path: &Path, output_path: &Path) -> std::io::Result<Self> {
        let input_file = File::open(input_path)?;
        let output_file = File::create(output_path)?;
        Ok(FileHandler{input_file,output_file})
    }
}
