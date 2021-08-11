use std::env;

#[derive(Debug)]
pub struct Argument{
    source:String,
    destination:String,
    byte_limit:u32,
    overrride:bool,
    output:Option<String>
}

impl Argument{

    // TODO: read all properties: 
    //      source: String => file/folder, 
    //      destination:String  => file/folder, 
    //      bytes limit => right now 50mb but should be nice to have this as parameter,
    //      override => by default true, 
    //      ouput => to change name if we are copy a folder
    //      more properties to add
    pub fn getArgs() -> Vec<String>{
        let mut args: Vec<String> = env::args().collect();

        args.remove(0);
        args
    }
}
