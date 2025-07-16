
#[derive(Clone, Debug)]
pub struct Dependencies {
    pub entires: Vec<Dependency>,
}


#[derive(Clone, Debug)]
pub struct Dependency {
    pub configuration_name: String,
    pub group: String,
    pub artifact: String,
    pub version: String,
}
