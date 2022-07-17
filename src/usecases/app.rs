use std::path::Path;
use crate::Argument;
use crate::common::errors::{AppResult, AppError};
use crate::consts;
use crate::tools::file::FileHelper;


pub fn copy_from_to(argument: Argument) -> AppResult<()> {

    let source_path = Path::new(argument.source());
    let destination_path = Path::new(argument.destination());

    if !source_path.exists(){
        return Err(AppError::new(&format!("Source is invalid, {}", consts::ERROR_FILE_NOT_EXIST)));
    }

    if !destination_path.exists() && destination_path.extension().is_none(){
        return Err(AppError::new(&format!("Destination folder not exist {}", consts::ACTION_NOT_SUPPORTED)));
    }

    let file_name = source_path.file_name().unwrap().to_str().unwrap();
    let from_str:String = source_path.to_str().unwrap().to_string();
    let mut to_str:String = destination_path.to_str().unwrap().to_string();

    if destination_path.is_dir(){        
        let new_path = destination_path.join(file_name);
        to_str = String::from(new_path.to_str().unwrap().clone());
    }

    println!("Starting to copy => \n source: {}\n destination: {}\n", &from_str, &to_str);
    
    FileHelper::copy(&from_str, &to_str, &argument.byte_limit()) 
}


// TODO: Should be nice to move this to external file
#[cfg(test)]
mod tests {
    use crate::{tools::argument::Argument};
    use crate::consts;

    use super::copy_from_to;

    #[test]
    fn when_source_is_invalid_should_error() {
        
        let data = Argument {
            source: String::from(""),
            destination: String::from("./src/main.rs"),
            byte_limit: None,
            override_files: false,
            new_name: None
        };

        let result = copy_from_to(data);
        
        assert!(result.is_err());
        let message = result.err().unwrap().to_string(); 
        assert!(message.ends_with(consts::ERROR_FILE_NOT_EXIST));
        assert!(message.starts_with("Source"));
    }

    #[test]
    fn when_destination_is_invalid_should_error() {
        
        let data = Argument {
            source: String::from("./src/main.rs"),
            destination: "some url".to_string(),
            byte_limit: None,
            override_files: false,
            new_name: None
        };

        let result = copy_from_to(data);
        
        assert!(result.is_err());
        let message = result.err().unwrap().to_string(); 
        assert!(message.ends_with(consts::ACTION_NOT_SUPPORTED));
        assert!(message.starts_with("Destination"));
    }

    #[test]
    fn when_source_destination_valid_should_be_success() {
        
        let data = Argument {
            source: String::from("./src/main.rs"),
            destination: String::from("./src/main.rs"),
            byte_limit: None,
            override_files: false,
            new_name: None
        };

        let result = copy_from_to(data);
        
        assert!(result.is_ok());
    }
}