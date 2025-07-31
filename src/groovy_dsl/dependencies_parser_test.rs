use super::*;

#[cfg(test)]


#[test]
pub fn parse_short_notation() {
let script = "dependencies {
    implementation 'some.group:artifact-name:1.2'
    api \"group:artifact:1.0\"
}";

let result = dependencies_parser::parse(script);

assert_eq!(result.entries.len(), 2);

assert_eq!(result.entries[0], Dependency {
    configuration_name: "implementation".to_string(),
    notation_type: NotationType::Short,
    dependency_notation: "'some.group:artifact-name:1.2'".to_string(),
    group: Option::Some(String::from("some.group")),
    artifact: Option::Some(String::from("artifact-name")),
    version: Option::Some(String::from("1.2"))
});

assert_eq!(result.entries[1], Dependency {
    configuration_name: "api".to_string(),
    notation_type: NotationType::Short,
    dependency_notation: "\"group:artifact:1.0\"".to_string(),
    group: Option::Some(String::from("group")),
    artifact: Option::Some(String::from("artifact")),
    version: Option::Some(String::from("1.0"))
});
}

