use crate::groovy_dsl::{
    BRACES_CLOSE, BRACES_OPEN, D_QOUTE, PARENTHESES_CLOSE, PARENTHESES_OPEN, QOUTE,
};

#[derive(Debug, Clone, Copy)]
pub struct Tokenizer<'a> {
    pub text: &'a str,
    cursor: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub lexeme: &'a str,
}

const DELIMETERS: [char; 4] = [
    BRACES_OPEN,
    BRACES_CLOSE,
    PARENTHESES_OPEN,
    PARENTHESES_CLOSE,
];

impl<'a> Token<'a> {
    pub fn  is_linebreak(&self) -> bool {
       self.lexeme.chars().all(Self::_is_linebreak)
    }
    fn _is_linebreak(ch: char) -> bool {
        println!("Test lb:{}", ch);
        matches!(ch, '\u{000A}' | '\u{000D}' | '\u{0085}' | '\u{2028}' | '\u{2029}')
    }
}
impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer { text, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        // println!("Cursor now: {:?}", self.cursor);
        if self.cursor >= self.text.len() {
            return None;
        }
        let text_chars = self.text[self.cursor..].chars();
        // println!("Not tokenized {:?}", text_chars);
        let mut next_char = None;
        for (offset, ch) in text_chars.enumerate() {
            // looking for end of string literal
            if let Some(nc) = next_char {
                if nc == ch {
                    let token = self.text[self.cursor..self.cursor + offset + 1].trim();
                    self.cursor += offset + 1;
                    return Some(Token { lexeme: token });
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
                return Some(Token { lexeme: token });
            }

            // exit on whitespace after some text
            let token = self.text[self.cursor..self.cursor + offset].trim();
            if ch.is_whitespace() && !token.is_empty() {
                self.cursor += offset + 1;
                return Some(Token { lexeme: token });
            }

            // string literal started, setup search for end
            if ch == D_QOUTE || ch == QOUTE {
                next_char = Some(ch);
                continue;
            }

            if offset + self.cursor + 1 == self.text.len() {
                let token = self.text[self.cursor..].trim();
                self.cursor += offset + 1;
                return Some(Token { lexeme: token });
            }
        }
        None
    }
}
