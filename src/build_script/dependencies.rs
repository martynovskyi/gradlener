use crate::groovy_dsl::{D_QOUTE, QOUTE};

#[derive(Clone, Debug)]
pub struct Dependencies<'a> {
    pub entries: Vec<Dependency<'a>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotationType {
    Short, 
    MapStyle,
    Advanced,
    Unknown
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dependency<'a>  {
    pub configuration_name: &'a str,
    pub notation_type: NotationType,
    pub dependency_notation: &'a str,
    pub group: Option<&'a str>,
    pub artifact: Option<&'a str>,
    pub version: Option<&'a str>,
}

impl<'a>  Dependency<'a>  {
    pub fn  short(configuration_name: &'a str, dependency_notation: &'a str) -> Dependency<'a> {
        let content = dependency_notation.trim_matches(|c| c == D_QOUTE || c == QOUTE);
        let mut notation_parts = content.split(':');
        Dependency {
            configuration_name: configuration_name,
            notation_type: NotationType::Short,
            dependency_notation: dependency_notation,
            group: notation_parts.next(),
            artifact: notation_parts.next(),
            version: notation_parts.next(),
        }
    }
}
