use std::collections::BTreeMap;

use lexer::Lexer;
use parser::{Parser, ParserError};

mod lexer;
mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Object(BTreeMap<String, String>)
}

pub fn parse(input: &str) -> Result<Value, ParserError> {
    match Lexer::new(input).tokenize() {
        Ok(tokens) => Parser::new(tokens).parse(),
        Err(e) => Err(ParserError::new(&e.msg)),
    }
}