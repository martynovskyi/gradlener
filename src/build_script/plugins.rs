
#[derive(Clone, Debug)]
pub struct Plugins {
    pub entries: Vec<Plugin>,
}

#[derive(Clone, Debug)]
pub struct Plugin {
    pub id: String,
    pub version: Option<String>,
    pub apply: Option<bool>,
}
