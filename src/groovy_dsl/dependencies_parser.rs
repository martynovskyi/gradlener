use crate::build_script::dependencies::*;

use crate::groovy_dsl::DEPENDENCIES_CONTAINER;
use crate::groovy_dsl::BRACES_OPEN;

pub fn parse(script: &str) -> Dependencies {
    
    if script.find(DEPENDENCIES_CONTAINER).is_some_and(|ind| ind != 0){
        panic!("Passed script is not a dependencies container");
    }
    println!("Input: {}", script);

    let mut entries: Vec<Dependency> = Vec::new();

    let mut raw: &str = &script[12..];

    let block_start = raw.find(BRACES_OPEN).unwrap_or_else(|| panic!("Bad Code"));


    Dependencies { entries: Vec::new()}
}
