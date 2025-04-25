use crate::token::Token;
use crate::token::TokenType;

pub struct Lexer{
    input: String,
    index: usize,
    next_index:usize,
    pub ch: char
}

impl Lexer {
    pub fn new(input: String) -> Lexer{
        let mut l = Lexer { input: input, index: 0, next_index: 0, ch: '\0' };
        l.read_char();
        return l;
    }


    pub fn read_char(&mut self){
        self.ch = if self.next_index >= self.input.chars().count(){
            '\0'
        }else {
            self.input.chars().nth(self.next_index).unwrap_or('\0')
        };
        self.index = self.next_index;
        self.next_index += 1;
    }
    pub fn next_token(&mut self) -> Token{
        let ret = match self.ch {
            '='     => Token { t: TokenType::ASSIGN, literal: self.ch },
            '+'     => Token { t: TokenType::PLUS, literal: self.ch },
            ','     => Token { t: TokenType::COMMA, literal: self.ch },
            '('     => Token { t: TokenType::LPAREN, literal: self.ch },
            ')'     => Token { t: TokenType::RPAREN, literal: self.ch },
            '{'     => Token { t: TokenType::LBRACE, literal: self.ch },
            '}'     => Token { t: TokenType::RBRACE, literal: self.ch },
            ';'     => Token { t: TokenType::SEMICOLON, literal: self.ch },
            '\0'    => Token { t: TokenType::EOF, literal: self.ch },
            _       => Token { t: TokenType::ILLEGAL, literal: self.ch },
        };
        self.read_char();
        return  ret;
    }
}


#[cfg(test)]
mod lexer_tests{
    use crate::token::Token;
    use crate::token::TokenType;

    use super::Lexer;

    #[test]
    fn test_next_token(){
        let input = String::from("+=(){},;");

        let tests = [
            Token{t:TokenType::PLUS, literal: '+'},
            Token{t:TokenType::ASSIGN, literal: '='},
            Token{t:TokenType::LPAREN, literal: '('},
            Token{t:TokenType::RPAREN, literal: ')'},
            Token{t:TokenType::LBRACE, literal: '{'},
            Token{t:TokenType::RBRACE, literal: '}'},
            Token{t:TokenType::COMMA, literal: ','},
            Token{t:TokenType::SEMICOLON, literal: ';'},
            Token{t:TokenType::EOF, literal: '\0'},
        ];

        let mut lex = Lexer::new(input);

        for test in tests.iter() {
            let tok = lex.next_token();
            println!("Testing: {:?} with type: {}", tok.t, tok.literal);
            assert!(tok.t == test.t, "There is an error with the Token Types");
            assert!(tok.literal == test.literal, "There is an error with the Token Literals");
        }
    }
}
