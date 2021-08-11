use std::{env, fmt::{Error, Result}, fs::{self, File}, io::{BufReader, Read, Seek, SeekFrom, Write}, path::Path};

//const KB = 1024;


pub struct FileHelper{
    //source:String,
    //destintion:String
}

impl FileHelper{
    
    /**
     */
    pub fn copy(source:String, destination:String) -> Result {
        // Take path
        let source_path = Path::new(&source);
        let destination_path = Path::new(&destination);

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

        let kb = 1024 * 5000; //50000;  // 50mb;

        //dbg!(&metadata);
        

        let mut total = metadata.len();
        let mut cursorPos = 0;
        let mut bytes =  0;

        while  total > 0{
            bytes = kb;
            if kb > total{
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
            println!("current copied: {}", cursorPos / 1024000);
            
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

