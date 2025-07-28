use crate::build_script::plugins::*;


pub fn parse(script: &str) -> Plugins {
    if script.find("plugins").is_some_and(|ind| ind != 0){
        panic!("Passed script is not a plugins container");
    }
    println!("Input: {}", script);

    let mut entries = Vec::new();

    let mut not_parsed: &str = script;

    while let Some(index) = not_parsed.find("id") {
        not_parsed = &not_parsed[(index + 2)..];
        let limit = not_parsed.find("id").unwrap_or(not_parsed.len());
        
        let plugin_spec = &not_parsed[..limit];
        println!("Spec: {}", plugin_spec);

        let id = parse_next_string_literal(plugin_spec);
        println!("{}", id);

        entries.push(Plugin {
            id: id.to_string(),
            version: parse_version(plugin_spec),
            apply: parse_apply(plugin_spec)
        });
    }

    Plugins { entries }
}

fn parse_version(spec_line: &str) -> Option<String> {
    spec_line.find("version")
.map(|i| parse_next_string_literal(&spec_line[(i + 7)..]).to_string())
}

fn parse_apply(spec_line: &str) -> Option<bool> {
    spec_line.find("apply")
.map(|i| parse_next_lexeme(&spec_line[(i + 5)..]).eq("true"))
}

pub fn parse_next_string_literal(script: &str) -> &str {
    let mut found = false;
    let mut s = 0usize;
    let mut e = 0usize;
    for (i, c) in script.chars().enumerate() {
            if c == '"' || c == '\'' {
                if !found {
                s = i + 1;
                found = true;
            } else {
                e = i;
                break;
            }
            }
    }
   &script[s..e]
}


pub fn parse_next_lexeme(script: &str) -> &str {
    let mut found = false;
    let mut s = 0usize;
    let mut e = 0usize;
        for (i, c) in script.chars().enumerate() {
            if !c.is_whitespace()  {
                if !found {
                s = i + 1;
                found = true;
            } else {
                e = i;
                break;
            }
            }
        }
   &script[s..e]
}
