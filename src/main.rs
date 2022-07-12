// Remember to include the mod and then use it
mod tools;
//mod copy;
mod common;
mod usecases;
mod consts;

use tools::argument::Argument;

// TODO: find a way to put all const in one file. so far we can call this with crate::MEGABYTE
pub const MEGABYTE: u32 = 1024000;

fn main() {
    let args = Argument::get();
    // Debug
    //dbg!(&args);

    // TODO:
    //  TODO: get required parameters
    //  TODO: call usecase to copy with options
    //  TODO: UseCase -> check if source exist && is file
    //  TODO: UseCase -> check if destination exist && is file, if exist clean file.
    //  TODO: UseCase -> copy file


    println!("Starting program");

    let mut errors = String::from("");

    if args.is_some() {
        let data = args.unwrap();

        match usecases::app::copy_from_to(data){
            Ok(()) => println!("Ok"),
            Err(e) => {
                println!("Error trying to read the message: {}", e);
                errors = e.to_string();
            }
        }
    }
    else {
        errors = "Missing parameters".to_string();
    }
    
    if !String::is_empty(&errors){
        // TODO: add more information about what logic is need
        // TODO: implement --help should be nice
        println!("Nothing to copy");
        println!("Usage: copyto [SOURCE] [DESTINATION]");
        println!("SOURCE : file/folder to copy");
        println!("DESTINATION: path to copy the file/folder");
    }
}
