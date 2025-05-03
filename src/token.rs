
#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,
    
    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    STAR,
    SLASH,
    GT,
    LT,

    COMMA,
    SEMICOLON,
    
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // cond
    EQ,
    NEQ,
    

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

}


#[derive(Debug)]
pub struct Token{
    pub t: TokenType,
    pub literal: String
}


impl Token {

    pub fn lookup_identifier(s: &str)-> TokenType{
        match s {
            "let"   => TokenType::LET,
            "fun"   => TokenType::FUNCTION,
            "true"   => TokenType::TRUE,
            "false"   => TokenType::FALSE,
            "if"   => TokenType::IF,
            "else"   => TokenType::ELSE,
            "ret"   => TokenType::RETURN,
            _       => TokenType::IDENT
        }
    }
}
