
use crate::build_script::dependencies::Dependencies;
use crate::build_script::plugins::Plugins;

#[derive(Clone, Debug)]
pub struct Project {
    pub plugins: Plugins,
    pub dependencies: Dependencies,
}
