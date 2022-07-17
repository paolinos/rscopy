### Description

This **Rust** will copy files adding limit to ensure the copy arount network.

If you are trying to use this, please DON'T!

### Run
Only copy files with absolute path
```
// To run 
cargo run {source_file} {destination_folder}

// Copy file with limit of 50Megas
cargo run {source_file} {destination_folder} -b 50 

// NOTE: Not implemente yet
cargo run {source_file} {destination_folder} -o {new_filename}
```

# TODO:
- Refactor Argument to read specific commands
- Read folders & file
- create folders
- copy files using '.partial' or something like that as extension, and remove it when file finished
- re-try copy bytes/file
- check permission before copy
- add async or parallel (or both async could be to copy bytes and parallel to run N times using CPU)?
- improve terminal feedback displaying file name and percentage to copy



### Build

**build windows app in linux**
```bash
# install dependency
apt-get install mingw-w64
# build debug version
cargo build --target x86_64-pc-windows-gnu
# build relesse version
cargo build --release --target x86_64-pc-windows-gnu
```

**examples**
```bash
# copy in windows
.\copy_file.exe "C:\some_image.jpg" "C:\temp\image.jpg"

# copy in linux
cargo run /tmp/image.jpg /user/tmp/destination

# local example
cargo run ./tmp/source/image1.jpg ./tmp/destination
```