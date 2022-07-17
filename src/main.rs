// Remember to include the mod and then use it
mod tools;
//mod copy;
mod common;
mod usecases;
mod consts;

use tools::argument::Argument;

fn main() {
    let args = Argument::get();
    // Debug
    //dbg!(&args);

    let mut errors = String::from("");

    if args.is_some() {
        let data = args.unwrap();

        match usecases::app::copy_from_to(data){
            Ok(()) => (),
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
        
        println!("\nUsage: copy_file [SOURCE] [DESTINATION] [OPTIONS]\n");
        println!("SOURCE : file/folder to copy");
        println!("DESTINATION: path to copy the file/folder");

        println!("\nOPTIONS:\n");
        println!("           -b integer     byterate to copy expressed in mb. (default 50mb)");
    }
}
