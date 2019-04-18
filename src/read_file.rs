use std::{env, fs};

pub fn get_file_to_string(mut args: env::Args) -> Result<String,&'static str> {
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a filename")
    };

    let content = match fs::read_to_string(filename) {
        Ok(value) => value,
        Err(_) => return Err("Failed to read file.")
    };

    Ok(content)

}