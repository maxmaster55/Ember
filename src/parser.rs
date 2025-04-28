use crate::{ast::{Expression, LetStatement, Program, Statement}, lexer::Lexer, token::{Token, TokenType}};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        return Self {
            lexer,
            current_token,
            peek_token 
        }
    }
    
    fn next_token(&mut self){
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut statements = vec![];

        while self.current_token.t != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Program { statements }
    }


    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.t {
            TokenType::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let name = match &self.current_token.t {
            TokenType::IDENT => self.current_token.literal.clone(),
            _ => return None,
        };

        self.next_token(); // expect '='
        if self.current_token.t != TokenType::ASSIGN {
            return None;
        }

        self.next_token();


        let value = self.parse_expression()?;

        self.next_token(); // skip semicolon
        Some(Statement::Let(LetStatement { name, value }))
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        if self.current_token.t == TokenType::INT {
            let value = self.current_token.literal.parse().ok()?;
            if self.peek_token.t != TokenType::SEMICOLON{
                self.parse_infix_expression()
            }else {
                Some(Expression::INT(value))
            }
        }else {
            None
        }

    }

    fn parse_infix_expression(&mut self) -> Option<Expression> {
        
        let left = match self.current_token.t {
            TokenType::INT => {
                let value = self.current_token.literal.parse().ok()?;
                Some(Expression::INT(value))
            }
            _ => None,
        }?;

        self.next_token(); // operator
        let operator = self.current_token.literal.clone();

        self.next_token(); // right-hand side
        let right = match self.current_token.t {
            TokenType::INT => {
                let value = self.current_token.literal.parse().ok()?;
                Some(Expression::INT(value))
            }
            _ => None,
        }?;

        Some(Expression::INFEX {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        println!("{:?}", program);
        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = vec!["x", "y", "foobar"];
        for (i, stmt) in program.statements.iter().enumerate() {
            match stmt {
                Statement::Let(let_stmt) => {
                    assert_eq!(let_stmt.name, expected_identifiers[i]);
                }
                _ => panic!("Expected LetStatement, got {:?}", stmt),
            }
        }
    }

    #[test]
    fn test_parse_infix_expressions() {
        let input = "
        let result = 5 + 10;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.name, "result");
                match &let_stmt.value {
                    Expression::INFEX { left, operator, right } => {
                        match **left {
                            Expression::INT(value) => assert_eq!(value, 5),
                            _ => panic!("Expected INT expression on the left"),
                        }
                        assert_eq!(operator, "+");
                        match **right {
                            Expression::INT(value) => assert_eq!(value, 10),
                            _ => panic!("Expected INT expression on the right"),
                        }
                    }
                    _ => panic!("Expected INFEX expression"),
                }
            }
            _ => panic!("Expected LetStatement"),
        }
    }

    #[test]
    fn test_invalid_let_statement() {
        let input = "
        let x 5;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        println!("{:?}", program);
    }
}
