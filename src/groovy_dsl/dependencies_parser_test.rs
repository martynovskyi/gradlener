use super::*;

#[cfg(test)]
#[test]
pub fn parse_short_notation() {
    let script = "dependencies {
    implementation 'some.group:artifact-name:1.2'
    api \"group:artifact:1.0\"
}";

    let result = dependencies_parser::parse(script);

    println!("{:#?}", result);

    assert_eq!(result.entries.len(), 2);

    assert_eq!(
        result.entries[0],
        Dependency {
            configuration_name: "implementation",
            notation_type: NotationType::Short,
            dependency_notation: "'some.group:artifact-name:1.2'",
            group: Some("some.group"),
            artifact: Some("artifact-name"),
            version: Some("1.2")
        }
    );

    assert_eq!(
        result.entries[1],
        Dependency {
            configuration_name: "api",
            notation_type: NotationType::Short,
            dependency_notation: "\"group:artifact:1.0\"",
            group: Some("group"),
            artifact: Some("artifact"),
            version: Some("1.0")
        }
    );
}
