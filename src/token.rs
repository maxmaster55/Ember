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

pub struct Token{
    pub t: TokenType,
    pub literal: char
}
impl Token {

}