
#[derive(Clone, Debug)]
pub struct Dependencies {
    pub entries: Vec<Dependency>,
}


#[derive(Clone, Debug)]
pub struct Dependency {
    pub configuration_name: String,
    pub group: String,
    pub artifact: String,
    pub version: String,
}
