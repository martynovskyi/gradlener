
use std::collections::HashMap;
pub struct ScriptIndex {



}

pub fn parse(script: &str) -> HashMap<String, (usize, usize)>  {
    let mut cursor = 0usize;
    let mut data: HashMap<String, (usize, usize)> = HashMap::new();


    let mut lexeme_s: usize = 0;
    let mut lexeme_e: usize = 0;
    let mut lexeme_read = false;
    let chars: Vec<char> = script.chars().collect();
    while  cursor < chars.len() {
        let c = chars[cursor];
        if c.is_whitespace() {
             if lexeme_read {
                lexeme_e = cursor;
                let block_after = check_block(&chars, cursor);
                if block_after.is_some() {
                    cursor = block_after.unwrap();
                }
                println!("{} - {:?}", cursor, &script[lexeme_s..lexeme_e]);
                data.insert(String::from(&script[lexeme_s..lexeme_e]), (lexeme_s, cursor));

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
    data
}

// return cursor position of end of block 
fn check_block(chars: &Vec<char>, mut cursor: usize) -> Option<usize> {
    let mut block_counter = 0;
    for c in &chars[cursor..] {
        if !c.is_whitespace() {
            if *c == '{' {
                block_counter +=1;
            }

            if *c == '}' {
                block_counter -= 1;
            }

            if block_counter == 0 {
                return Option::Some(cursor);
            }
        }
        cursor += 1;
    }
    Option::None

}
