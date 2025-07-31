use crate::build_script::dependencies::*;


pub fn parse(script: &str) -> Dependencies {

    if script.find("dependencies").is_some_and(|ind| ind != 0){
        panic!("Passed script is not a dependencies container");
    }
    println!("Input: {}", script);

    let mut entries: Vec<Dependency> = Vec::new();

    let mut not_parsed: &str = script;


    Dependencies { entries: Vec::new()}
}
