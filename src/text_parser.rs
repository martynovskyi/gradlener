

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
