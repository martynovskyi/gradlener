use super::*;

#[cfg(test)]


#[test]
pub fn parse() {
    let script = "plugins {
id 'plugin.a' version '1.0'
id \"plugin.x\" version \"1.0-rc\"
}";
    let result = plugin_parser::parse(script);

    assert_eq!(result.entries.len(), 2);

    assert_eq!(result.entries[0], Plugin {
        id: "plugin.a".to_string(),
        version: Option::Some("1.0".to_string()),
        apply: Option::None});
    assert_eq!(result.entries[1], Plugin {
        id: "plugin.x".to_string(),
        version: Option::Some("1.0-rc".to_string()),
        apply: Option::None});
}

