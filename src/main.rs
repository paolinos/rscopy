// Remember to include the mod and then use it
mod tools;
mod copy;
use tools::argument::Argument;

use crate::tools::file::FileHelper;

fn main() {
    // TODO: change Argument implementation to parse and validate all parameters
    let args = Argument::getArgs();
    //println!("{:?}", args);

    // TODO:
    //  source file
    //  destination
    // TODO: Update this with the new Argument 
    if args.len() == 2{
        
        // TODO: refactor this, Copy should be able to read folder/file
        // Copy
        match FileHelper::copy(args[0].to_string(), args[1].to_string()){
            Ok(()) => println!("Ok"),
            Err(e) => println!("Error trying to read the message: {}", e)
        };
       
    }else{
        // TODO: add more information about what logic is need
        // TODO: implement --help should be nice
        println!("Nothing to copy");
        println!("Usage: copyto [SOURCE] [DESTINATION]");
        println!("SOURCE : file/folder to copy");
        println!("DESTINATION: path to copy the file/folder");
    }
}
