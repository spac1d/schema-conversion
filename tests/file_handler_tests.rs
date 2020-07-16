extern crate schema_conversion;
#[cfg(test)]
mod tests{
    use schema_conversion::file_handler::FileHandler;
    use std::path::Path;

    #[test]
    fn open_files(){
        let input_file = Path::new("examples/readtestfile.txt");
        let output_file = Path::new("examples/writetestfile.txt");
        let file_handler : Result<FileHandler,_> = FileHandler::new(input_file,output_file);
        assert!(file_handler.is_ok());
    }

}