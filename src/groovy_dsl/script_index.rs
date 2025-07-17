
pub struct ScriptIndex {



}

pub fn parse(script: &str) -> ScriptIndex {
    let mut cursor = 0usize;
   
    let mut lexeme_s: usize = 0;
    let mut lexeme_e: usize = 0;
    let mut lexeme_read = false;
    let chars: Vec<char> = script.chars().collect();
    for c in script.chars() {
        if c.is_whitespace() {
             if lexeme_read {
                lexeme_e = cursor;
                println!("{} - {:?}", cursor, &script[lexeme_s..lexeme_e]);
            }
            lexeme_s = 0;
            lexeme_e = 0;
            lexeme_read = false;
            cursor += 1;
            continue;
        }

        if !lexeme_read {
            lexeme_s = cursor;
            lexeme_read = true;
        } else {
            lexeme_e = cursor;
        }
        cursor += 1;
    }

    ScriptIndex { }
} 
