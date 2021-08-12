use std::{env, fmt::{Error, Result}, fs::{self, File}, io::{BufReader, Read, Seek, SeekFrom, Write}, path::Path};

//const KB = 1024;


pub struct FileHelper{
    //source:String,
    //destintion:String
}

impl FileHelper{
    
    /**
     */
    pub fn copy(source:&String, destination:&String, limit_bytes:&Option<u32>) -> Result {
        // Take path
        let source_path = Path::new(source);
        let destination_path = Path::new(destination);
        
        let mut megas: u64 = 50 * crate::MEGABYTE as u64;

        if limit_bytes.is_some() {
            megas = limit_bytes.unwrap() as u64;
        }

        // TODO: need to clear/create empty file, because then we're appending bytes to file
        // TODO: destination file should change the extension, like '.partial' and remove it after copy all files
        /*
        fs::remove_file(destination_path);
        if !destination_path.exists(){
            // Create if not exit
            File::create(destination_path);
        }
        */
        FileHelper::clear_file(destination_path);


        println!("The current directory is {:?}", source_path.join(&source) );

        let f = File::open(source_path).expect("no file found");
        let metadata = f.metadata().expect("File error");
        //dbg!(&metadata);

        

        let mut total = metadata.len();
        let mut cursorPos = 0;
        let mut bytes =  0;

        while  total > 0{
            bytes = megas;
            if megas > total{
                bytes = total;
            }

            println!("Total bytes to copy:{}, Total copied bytes:{}, to copy:{}, ", total, &cursorPos, &bytes);

            // Create buffer an move the starting position of the stream
            let mut buffer = vec![0; bytes as usize];
            let mut reader = BufReader::new(&f);
            // TODO: Add validation here to retry
            reader.seek(SeekFrom::Start(cursorPos));
            reader.read(&mut buffer);

            cursorPos += bytes;
            total -= bytes;
            println!("current copied: {}", cursorPos / (crate::MEGABYTE as u64));
            
            // TODO: this options are to append bytes to existing file. 
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(destination_path)
                .unwrap();

            file.write(&buffer).expect("Error trying to copy file");
        }

        Ok(())        
    }

    /**
     * Delete file if exist or create a new empty file
     */
    fn clear_file(path:&Path) {

        if path.exists() {
            // Delete file
            fs::remove_file(path).unwrap();
        }
        File::create(path).unwrap();
    }
}

