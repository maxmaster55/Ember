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
    
    pub  fn read_identifier(&mut self) -> String {
        let s = self.index;
        while Lexer::is_letter(self.ch) {
            println!("----> {}", self.ch);

            self.read_char();            
        }

        let e = self.index;

        return String::from(&self.input[s..e]); // from start index to end
    }

    pub  fn read_number(&mut self) -> String {
        let s = self.index;
        while Lexer::is_digit(self.ch) {
            self.read_char();            
        }

        let e = self.index;

        return String::from(&self.input[s..e]); // from start index to end
    }


    
    pub fn skip_spaces(&mut self){
        let space_types = [' ', '\t', '\n'];
        while self.ch == ' '{
            self.read_char();
        }
    }

    fn is_letter(ch : char) -> bool{
        return ch.is_ascii_alphabetic() || ch == '_';
    
    }
    fn is_digit(ch : char) -> bool{
        return ch <= '9' && ch >= '0';
    
    }

    pub fn next_token(&mut self) -> Token{
        self.skip_spaces();
        let tok:Token = match self.ch {
            '='     => Token { t: TokenType::ASSIGN, literal: String::from(self.ch) },
            '+'     => Token { t: TokenType::PLUS, literal: String::from(self.ch) },
            ','     => Token { t: TokenType::COMMA, literal: String::from(self.ch) },
            '('     => Token { t: TokenType::LPAREN, literal: String::from(self.ch) },
            ')'     => Token { t: TokenType::RPAREN, literal: String::from(self.ch) },
            '{'     => Token { t: TokenType::LBRACE, literal: String::from(self.ch) },
            '}'     => Token { t: TokenType::RBRACE, literal: String::from(self.ch) },
            ';'     => Token { t: TokenType::SEMICOLON, literal: String::from(self.ch) },
            '\0'    => Token { t: TokenType::EOF, literal: String::from(self.ch) },
            _       => if Lexer::is_letter(self.ch) {
                        let word: String = self.read_identifier();
                        let tok_type: TokenType = Token::lookup_identifier(&word);
                        Token { t: tok_type, literal: word }
                    }else if Lexer::is_digit(self.ch){
                        let num: String = self.read_number();
                        Token { t: TokenType::INT, literal: num }
                    }else {
                        Token { t: TokenType::ILLEGAL, literal: String::from(self.ch)  }
                    }
                    
        };
        self.read_char();
        return  tok;
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
            Token{t:TokenType::PLUS, literal: String::from("+")},
            Token{t:TokenType::ASSIGN, literal: String::from("=")},
            Token{t:TokenType::LPAREN, literal: String::from("(")},
            Token{t:TokenType::RPAREN, literal: String::from(")")},
            Token{t:TokenType::LBRACE, literal: String::from("{")},
            Token{t:TokenType::RBRACE, literal: String::from("}")},
            Token{t:TokenType::COMMA, literal: String::from(",")},
            Token{t:TokenType::SEMICOLON, literal: String::from(";")},
            Token{t:TokenType::EOF, literal: String::from("\0")},
        ];

        let mut lex = Lexer::new(input);

        for test in tests.iter() {
            let tok = lex.next_token();
            println!("Testing: {:?} with type: {}", tok.t, tok.literal);
            assert!(tok.t == test.t, "There is an error with the Token Types");
            assert!(tok.literal == test.literal, "There is an error with the Token Literals");
        }
    }

    #[test]
    fn test_lookup_identifier(){
        let input = String::from("let    x = 5");
        
        let tests = [
            Token{t:TokenType::LET, literal: String::from("let")},
            Token{t:TokenType::IDENT, literal: String::from("x")},
            Token{t:TokenType::ASSIGN, literal: String::from("=")},
            Token{t:TokenType::INT, literal: String::from("5")},
            Token{t:TokenType::EOF, literal: String::from("\0")},
        ];

        let mut lex = Lexer::new(input);

        for test in tests.iter() {
            let tok = lex.next_token();
            println!("{tok:?}");
            println!("Testing: {:?} with type: {}", tok.t, tok.literal);
            assert!(tok.t == test.t, "There is an error with the Token Types");
            assert!(tok.literal == test.literal, "There is an error with the Token Literals");
        }
    }
    
}
