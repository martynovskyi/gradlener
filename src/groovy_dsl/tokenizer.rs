pub struct Tokenizer<'a> {
   pub text: &'a str,
    cursor: usize
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub lexeme: &'a str,
}

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
        for (offset, ch) in text_chars.enumerate() {
            if ch.is_whitespace() && offset > 0 {
                let token = self.text[self.cursor..self.cursor + offset].trim();
                self.cursor += offset + 1;
                return Option::Some(Token { lexeme: token});
            }
            if offset + self.cursor + 1 == self.text.len(){
                let token = self.text[self.cursor..].trim();
                self.cursor += offset + 1;
                return Option::Some(Token { lexeme: token});
            }
        }
        Option::None
    }   
}
