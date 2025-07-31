

#[derive(Clone, Debug)]
pub struct Dependencies {
    pub entries: Vec<Dependency>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NotationType {
    Short, 
    MapStyle,
    Advanced,
    Unknown
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dependency {
    pub configuration_name: String,
    pub notation_type: NotationType,
    pub dependency_notation: String,
    pub group: Option<String>,
    pub artifact: Option<String>,
    pub version: Option<String>,
}
