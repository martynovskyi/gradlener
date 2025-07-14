
mod script_file;
use script_file::read_build_file;

mod cli_struct;
use clap::Parser;
use cli_struct::Args;

mod script_structure;

mod groovy_dsl;


fn main() {
    let args = Args::parse();
    println!("Args: {:?}", args); 

    let content = match read_build_file(&args.file) {
        Some(c) => c,
        None => { println!("Error: build script file [{}] not exist", args.file); std::process::exit(0); }
    };

    println!("Build script: {}", content);

    groovy_dsl::parse(&content);

}

