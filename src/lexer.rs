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
        self.ch =self.input.chars().nth(self.next_index).unwrap_or('\0');
        
        self.index = self.next_index;
        self.next_index += 1;
    }
    
    pub  fn read_identifier(&mut self) -> String {
        let s = self.index;
        while Lexer::is_letter(self.ch) {
            
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
        while space_types.contains(&self.ch){
            self.read_char();
        }
    }

    fn is_letter(ch : char) -> bool{
        return ch.is_ascii_alphabetic() || ch == '_';
    
    }
    fn is_digit(ch : char) -> bool{
        return ch <= '9' && ch >= '0';
    
    }
    pub fn peek_char(&self)-> char{
        return self.input.chars().nth(self.next_index).unwrap_or('\0');
    }

    pub fn next_token(&mut self) -> Token{
        self.skip_spaces();
        let tok:Token = match self.ch {
            '+'     => Token { t: TokenType::PLUS, literal: String::from(self.ch) },
            '-'     => Token { t: TokenType::MINUS,literal: String::from(self.ch) },
            '*'     => Token { t: TokenType::STAR,literal: String::from(self.ch) },
            '/'     => Token { t: TokenType::SLASH,literal: String::from(self.ch) },
            '>'     => Token { t: TokenType::GT,literal: String::from(self.ch) },
            '<'     => Token { t: TokenType::LT,literal: String::from(self.ch) },
            ','     => Token { t: TokenType::COMMA, literal: String::from(self.ch) },
            '('     => Token { t: TokenType::LPAREN, literal: String::from(self.ch) },
            ')'     => Token { t: TokenType::RPAREN, literal: String::from(self.ch) },
            '{'     => Token { t: TokenType::LBRACE, literal: String::from(self.ch) },
            '}'     => Token { t: TokenType::RBRACE, literal: String::from(self.ch) },
            ';'     => Token { t: TokenType::SEMICOLON, literal: String::from(self.ch) },
            '='     => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token { t: TokenType::EQ, literal: String::from("==") }
                }else {
                    Token { t: TokenType::ASSIGN, literal: String::from(self.ch) }
                }
            },
            '!'     => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token { t: TokenType::NEQ, literal: String::from("!=") }
                }else {
                    Token { t: TokenType::BANG, literal: String::from(self.ch) }
                }
            },
            '\0'    => Token { t: TokenType::EOF, literal: String::from(self.ch) },
            _       => if Lexer::is_letter(self.ch) {
                        let word: String = self.read_identifier();
                        let tok_type: TokenType = Token::lookup_identifier(&word);
                        return Token { t: tok_type, literal: word }
                    }else if Lexer::is_digit(self.ch){
                        let num: String = self.read_number();
                        return Token { t: TokenType::INT, literal: num }
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
        let input = String::from("let    x ==    5");
        
        let tests = [
            Token{t:TokenType::LET, literal: String::from("let")},
            Token{t:TokenType::IDENT, literal: String::from("x")},
            Token{t:TokenType::EQ, literal: String::from("==")},
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


    #[test]
    fn test_arithmetic_operators() {
        let input = String::from("+-*/");

        let tests = [
            Token { t: TokenType::PLUS, literal: String::from("+") },
            Token { t: TokenType::MINUS, literal: String::from("-") },
            Token { t: TokenType::STAR, literal: String::from("*") },
            Token { t: TokenType::SLASH, literal: String::from("/") },
            Token { t: TokenType::EOF, literal: String::from("\0") },
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
    fn test_comparison_operators() {
        let input = String::from(">= <= != ==");

        let tests = [
            Token { t: TokenType::GT, literal: String::from(">") },
            Token { t: TokenType::ASSIGN, literal: String::from("=") },
            Token { t: TokenType::LT, literal: String::from("<") },
            Token { t: TokenType::ASSIGN, literal: String::from("=") },
            Token { t: TokenType::NEQ, literal: String::from("!=") },
            Token { t: TokenType::EQ, literal: String::from("==") },
            Token { t: TokenType::EOF, literal: String::from("\0") },
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
    fn test_identifiers_and_keywords() {
        let input = String::from("let five = 5;;;;");

        let tests = [
            Token { t: TokenType::LET, literal: String::from("let") },
            Token { t: TokenType::IDENT, literal: String::from("five") },
            Token { t: TokenType::ASSIGN, literal: String::from("=") },
            Token { t: TokenType::INT, literal: String::from("5") },
            Token { t: TokenType::SEMICOLON, literal: String::from(";") },
            Token { t: TokenType::SEMICOLON, literal: String::from(";") },
            Token { t: TokenType::SEMICOLON, literal: String::from(";") },
            Token { t: TokenType::SEMICOLON, literal: String::from(";") },
            Token { t: TokenType::EOF, literal: String::from("\0") },
        ];

        let mut lex = Lexer::new(input);

        for (i, test) in tests.iter().enumerate() {
            let tok = lex.next_token();
            println!("Test {}: Expected {:?}, Got {:?}", i, test, tok);
            assert!(tok.t == test.t, "Error with Token Type at index {}: Expected {:?}, Got {:?}", i, test.t, tok.t);
            assert!(tok.literal == test.literal, "Error with Token Literal at index {}: Expected {:?}, Got {:?}", i, test.literal, tok.literal);
        }
    }

    #[test]
    fn test_illegal_characters() {
        let input = String::from("@#$");

        let tests = [
            Token { t: TokenType::ILLEGAL, literal: String::from("@") },
            Token { t: TokenType::ILLEGAL, literal: String::from("#") },
            Token { t: TokenType::ILLEGAL, literal: String::from("$") },
            Token { t: TokenType::EOF, literal: String::from("\0") },
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
