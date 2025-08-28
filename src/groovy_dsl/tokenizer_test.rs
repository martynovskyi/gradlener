use super::*;

#[cfg(test)]

#[test]
pub fn new_tokenizer() {
    let text = "1 2 3";

    let tokenizer = tokenizer::Tokenizer::new(text);
    assert_eq!(tokenizer.text, text);
}

#[test]
pub fn tokenize_block() {
    let text = "block { }";

    let mut tokenizer = tokenizer::Tokenizer::new(text);

    assert_eq!(tokenizer.next(),
        Option::Some(tokenizer::Token{ lexeme: "block" }));
    assert_eq!(tokenizer.next(),
        Option::Some(tokenizer::Token{ lexeme: "{" }));
    assert_eq!(tokenizer.next(),
        Option::Some(tokenizer::Token{ lexeme: "}" }));
    assert_eq!(tokenizer.next(), Option::None);
}


#[test]
pub fn tokenize_assigment_expression_with_string_literal() {
    let text = "description = \"some text\"";

    let mut tokenizer = tokenizer::Tokenizer::new(text);

    assert_eq!(tokenizer.next(),
        Option::Some(tokenizer::Token{ lexeme: "description" }));
    assert_eq!(tokenizer.next(),
        Option::Some(tokenizer::Token{ lexeme: "=" }));
    assert_eq!(tokenizer.next(),
        Option::Some(tokenizer::Token{ lexeme: "\"some text\"" }));
    assert_eq!(tokenizer.next(), Option::None);
}
