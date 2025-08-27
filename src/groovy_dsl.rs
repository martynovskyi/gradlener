
use crate::build_script::project::Project;
use crate::build_script::plugins::*;
use crate::build_script::dependencies::*;

use std::collections::HashMap;

mod tokenizer;
mod tokenizer_test;
mod script_index;
mod script_index_test;
mod plugin_parser;
mod plugin_parser_test;
mod dependencies_parser;
mod dependencies_parser_test;


pub const PLUGINS_CONTAINER: &str = "plugins";
pub const DEPENDENCIES_CONTAINER: &str = "dependencies";
pub const BLOCK_START: &str = "{";
pub const BLOCK_END: &str = "}";

pub fn parse(script: &str) -> Project {

    let script_index: HashMap<String, (usize, usize)> = script_index::parse(script);

    println!("Index: {:?}", script_index);



    let plugins: Plugins = match script_index.get(PLUGINS_CONTAINER) {
        Some(pair) => { 
            let (start, end): (usize, usize) = *pair;
            plugin_parser::parse(&script[start..end])
        },
        None => Plugins { entries: Vec::new() }
    };

    let dependencies: Dependencies =  match script_index.get(DEPENDENCIES_CONTAINER) {
        Some(pair) => { 
            let (start, end): (usize, usize) = *pair;
            dependencies_parser::parse(&script[start..end])
        },
        None => Dependencies { entries: Vec::new() }
    };
    Project{ plugins, dependencies, }
}

