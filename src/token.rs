use std::collections::HashMap;

use crate::lexer::Lexer;



#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,
    
    ASSIGN,
    PLUS,
    
    COMMA,
    SEMICOLON,
    
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    
    FUNCTION,
    LET
}


#[derive(Debug)]
pub struct Token{
    pub t: TokenType,
    pub literal: String
}


impl Token {

    pub fn lookup_identifier(s: &str)-> TokenType{
        match s {
            "let" => TokenType::LET,
            "fun" => TokenType::FUNCTION,
            _ => TokenType::IDENT
        }
    }
}
