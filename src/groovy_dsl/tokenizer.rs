use crate::groovy_dsl::{D_QOUTE, QOUTE, BRACES_OPEN, BRACES_CLOSE, PARENTHESES_OPEN, PARENTHESES_CLOSE};

pub struct Tokenizer<'a> {
   pub text: &'a str,
    cursor: usize
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub lexeme: &'a str,
}

const DELIMETERS: [char; 4] = [BRACES_OPEN, BRACES_CLOSE, PARENTHESES_OPEN, PARENTHESES_CLOSE];

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
        text, cursor: 0
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.cursor == self.text.len() {
           return Option::None;
        }
        let text_chars = self.text[self.cursor..].chars();
        let mut next_char = Option::None;
        for (offset, ch) in text_chars.enumerate() {
            // looking for end of string literal
            if let Some(nc) = next_char { if nc == ch {
                let token = self.text[self.cursor..self.cursor + offset + 1].trim();
                self.cursor += offset + 1;
                return Option::Some(Token { lexeme: token});
            }
                continue;
            }

            if DELIMETERS.contains(&ch) {
                let mut token = self.text[self.cursor..self.cursor + offset].trim();
                if token.is_empty() {
                    token = self.text[self.cursor..self.cursor + offset + 1].trim();
                    self.cursor += offset + 1;
                } else {
                    self.cursor += offset;
                }
                return Option::Some(Token { lexeme: token});
            }

            // exit on whitespace after some text
            if ch.is_whitespace() && offset > 0 {
                let token = self.text[self.cursor..self.cursor + offset].trim();
                self.cursor += offset + 1;
                return Option::Some(Token { lexeme: token});
            }

            // string literal started, setup search for end
            if ch == D_QOUTE || ch == QOUTE {
                next_char = Option::Some(ch);    
                continue;
            }

            if offset + self.cursor + 1 == self.text.len() {
                println!("End of text");
                let token = self.text[self.cursor..].trim();
                self.cursor += offset + 1;
                return Option::Some(Token { lexeme: token});
            }
        }
        Option::None
    }   
}
