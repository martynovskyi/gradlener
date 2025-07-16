
use crate::build_script::project::Project;
use crate::build_script::plugins::*;
use crate::build_script::dependencies::*;

mod script_index;

pub fn parse(script: &str) -> Project {

    let script_index = script_index::parse(script);

    let plugins: Plugins = Plugins {
        entries: Vec::new(),
    };

    let dependencies: Dependencies = Dependencies {
        entires: Vec::new()
    };

    Project{ plugins, dependencies, }
}
