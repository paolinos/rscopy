### Description

This **Rust** will copy files adding limit to ensure the copy arount network.

If you are trying to use this, please DON'T!

### Run
Only copy files with absolute path
```
// To run 
cargo run {source_file} {destination_file}
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
