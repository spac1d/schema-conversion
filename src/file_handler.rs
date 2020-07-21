use crate::json_parser::JsonParser;

use std::fs::{File};
use std::path::Path;
use std::io::{Lines, BufReader, BufRead};
use std::fs;

pub struct FileHandler {
    pub output_file: File,
    parser: JsonParser,
}

impl FileHandler{
    pub fn new(input_path: &Path, output_path: &Path) -> std::io::Result<Self> {
        let input_text = fs::read_to_string(input_path)?;
        let output_file = File::create(output_path)?;
        let parser = JsonParser{input_text, current_line: 0};
        Ok(FileHandler{output_file,parser})
    }
    pub fn parse(&mut self) -> Result<bool,String>{
        self.parser.parse()
    }
}
