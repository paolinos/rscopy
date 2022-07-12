use std::{fs::{self, File, metadata}, io::{BufReader, Read, Seek, SeekFrom, Write}, path::Path};

use crate::common::errors;


//const KB = 1024;


pub struct FileHelper{
    //source:String,
    //destintion:String
}

impl FileHelper{
    
    /**
     */
    pub fn copy(source:&str, destination:&str, limit_bytes:&Option<u32>) -> errors::AppResult<()> {
        // Take path
        let source_path = Path::new(source);
        let destination_path = Path::new(destination);

        if source_path.is_dir() {
            return Err(errors::AppError::new("Unsuported directory copy"));
        }

        if destination_path.is_dir() {
            return Err(errors::AppError::new("Unsuported directory destination. please add a file name"));
        }
        
        
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
        let mut cursor_pos = 0;
        //let mut bytes =  0;

        while  total > 0{
            let mut bytes = megas;
            if megas > total{
                bytes = total;
            }

            println!("Total bytes to copy:{}, Total copied bytes:{}, to copy:{}, ", total, &cursor_pos, &bytes);

            // Create buffer an move the starting position of the stream
            let mut buffer = vec![0; bytes as usize];
            let mut reader = BufReader::new(&f);
            // TODO: Add validation here to retry
            reader.seek(SeekFrom::Start(cursor_pos)).unwrap();
            reader.read(&mut buffer).unwrap();

            cursor_pos += bytes;
            total -= bytes;
            println!("current copied: {}", cursor_pos / (crate::MEGABYTE as u64));
            
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
    pub fn clear_file(path:&Path) {

        if path.exists() && path.is_file() {
            // Delete file
            fs::remove_file(path).unwrap();
        }
        File::create(path).unwrap();
    }
}

