use std::env;

#[derive(Debug)]
pub struct Argument{
    pub source:String,
    pub destination:String,
    pub byte_limit:Option<u32>,
    pub override_files:bool,
    pub new_name:Option<String>
}

impl Argument{

    /// get arguments from command line
    /// the required arguments are the frist two {source} {destination} the other are optionals
    /// by default override is true
    /// ```
    /// let args = Argument::get();
    /// if args.is_some(){
    ///     // Here we can read all props
    ///     args.source();
    ///     args.destination();
    ///     args.byte_limit();
    ///     args.new_name();
    ///     args.override_files();
    /// }
    /// ```
    pub fn get() -> Option<Argument> {
        let mut args: Vec<String> = env::args().collect();
        // remove 0 index, always is the path of the file
        args.remove(0);

        if args.len() < 2 {
            return None;
        }

        let mut source = String::from("");
        let mut destination = String::from("");
        let mut bytes:Option<u32> = None;
        let mut new_name: Option<String> = None;
        // TODO: not implemented yet
        let override_files:bool = true;
        
        let total = args.len();
        let mut pos: usize = 0;
        let mut iter = args.into_iter();
        // NOTE: not sure if is the bes way to read props
        while pos < total {
            match pos {
                0 => source = iter.next().unwrap(),
                1 => destination = iter.next().unwrap(),
                _ => {
                    let prop = iter.next().unwrap();
                    let propstr = prop.as_str();
                    match propstr {
                        "-b" => {
                            bytes = Some(iter.next().unwrap().parse::<u32>().unwrap() * crate::MEGABYTE);
                            pos += 1;
                        },
                        "-o" => {
                            new_name = Some(iter.next().unwrap());
                            pos += 1;
                        }
                        _ => {}
                    }
                }
            }
            pos += 1;
        }


        Some(Argument {
            source,
            destination,
            byte_limit: bytes,
            new_name,
            override_files
        })
    }

    pub fn source(&self) -> &String {
        &self.source
    }

    pub fn destination(&self) -> &String {
        &self.destination
    }

    pub fn byte_limit(&self) -> &Option<u32> {
        &self.byte_limit
    }

    pub fn new_name(&self) -> &Option<String> {
        &self.new_name
    }
    
    pub fn override_files(&self) -> &bool {
        &self.override_files
    }
}
