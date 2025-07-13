
mod cli_struct;
use clap::Parser;
use cli_struct::Args;

fn main() {
    let args = Args::parse();
    println!("Args: {:?}", args);
}
