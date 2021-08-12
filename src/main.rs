// Remember to include the mod and then use it
mod tools;
mod copy;
use tools::argument::Argument;

use tools::file::FileHelper;

// TODO: find a way to put all const in one file. so far we can call this with crate::MEGABYTE
pub const MEGABYTE: u32 = 1024000;

fn main() {
    let args = Argument::get();    
    // Debug
    //dbg!(&args);

    if args.is_some() {
        let data = args.unwrap();
        
        // TODO: this should receive the args
        match FileHelper::copy(data.source(), data.destination(), data.byte_limit()){
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
