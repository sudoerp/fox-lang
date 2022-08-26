
//use std::usize;

use crate::token_type::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object{
    Num(f32),
    str(String),
    Bool(bool),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token{
     ttype: TokenType,
     lexeme: String,
     literal: Option<Object>,
     line: usize
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line:usize) -> Token{
        Token {
            ttype,
            lexeme,
            literal,
            line
        }
    }
    pub fn eof(line:usize) -> Token{
        Token { ttype: TokenType::EOF, lexeme: "".to_owned(), literal: None, line }
    }
}
