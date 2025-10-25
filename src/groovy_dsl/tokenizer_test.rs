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

    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "block" }));
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "{" }));
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "}" }));
    assert_eq!(tokenizer.next(), None);
}

#[test]
pub fn tokenize_assigment_expression_with_string_literal() {
    let text = "description = \"some text\"";

    let mut tokenizer = tokenizer::Tokenizer::new(text);

    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "description"
        })
    );
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "=" }));
    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "\"some text\""
        })
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
pub fn tokenize_assigment_expression_with_string_literal_with_comma() {
    let text = "var = 'some text',";

    let mut tokenizer = tokenizer::Tokenizer::new(text);

    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "var" }));
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "=" }));
    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "'some text'"
        })
    );
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "," }));
    assert_eq!(tokenizer.next(), Option::None);
}

#[test]
pub fn tokenize_method_with_param() {
    let text = "implementation(\"some:dependency\")";

    let mut tokenizer = tokenizer::Tokenizer::new(text);

    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "implementation"
        })
    );
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "(" }));
    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "\"some:dependency\""
        })
    );
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: ")" }));
    assert_eq!(tokenizer.next(), None);
}

#[test]
pub fn tokenize_with_linebreak() {
    let text = "implementation(\"some:dependency\")
    api 'some:dependency'";

    let mut tokenizer = tokenizer::Tokenizer::new(text);

    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "implementation"
        })
    );
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: "(" }));
    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "\"some:dependency\""
        })
    );
    assert_eq!(tokenizer.next(), Some(tokenizer::Token { lexeme: ")" }));
    
    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "api"
        })
    );


    assert_eq!(
        tokenizer.next(),
        Some(tokenizer::Token {
            lexeme: "'some:dependency'"
        })
    );

    assert_eq!(tokenizer.next(), None);
}
