
use std::path::Path;
use std::fs;
mod cli_struct;
use clap::Parser;
use cli_struct::Args;

fn main() {
    let args = Args::parse();
    println!("Args: {:?}", args);

    let build_script = read_build_file(&args.file); 
}

fn read_build_file(file: &String) -> String {
    let path = Path::new(&file);
    if path.exists() {  
        let contents = fs::read_to_string(path).unwrap();
        println!("{}", contents);
        return contents;
    }
    return String::new();
}
