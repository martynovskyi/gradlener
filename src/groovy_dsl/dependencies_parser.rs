use crate::build_script::dependencies::*;

use crate::groovy_dsl::{BRACES_CLOSE, BRACES_OPEN, DEPENDENCIES_CONTAINER};
use crate::groovy_dsl::tokenizer::Tokenizer;

pub fn parse( script: &str) -> Dependencies {

    let mut tokenizer = Tokenizer::new(script);
    println!("Input: {}", script);

    if let Some (block_name) = tokenizer.next() { if block_name.lexeme != DEPENDENCIES_CONTAINER {
        panic!("Passed script is not a dependencies container");
    }} else {
        panic!("Empty script passed");
    }

    if let Some (open_block) = tokenizer.next() { if open_block.lexeme != BRACES_OPEN.to_string() {
            panic!("Passed script is not a block");
        }
    }
    let mut entries: Vec<Dependency> = Vec::new();
    let mut token_op =  tokenizer.next();
    while token_op.is_some() && token_op.unwrap().lexeme != BRACES_CLOSE.to_string() {
        entries.push(Dependency::short(token_op.unwrap().lexeme, tokenizer.next().unwrap().lexeme));
        token_op = tokenizer.next();
    }
    Dependencies { entries: entries }
}
