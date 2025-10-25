
use crate::build_script::dependencies::Dependencies;
use crate::build_script::plugins::Plugins;

#[derive(Clone, Debug)]
pub struct Project<'a> {
    pub plugins: Plugins,
    pub dependencies: Dependencies<'a>,
}
