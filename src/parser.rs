use crate::{ast::{Expression, LetStatement, Program, Statement}, lexer::Lexer, token::{Token, TokenType}};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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

    pub fn parse_program(&mut self) -> Program {
        let mut statements = vec![];

        while self.current_token.t != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }else {
                println!("Error");
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

        self.next_token();
        match self.current_token.t {
            TokenType::SEMICOLON => {},
            _ => return None
        }
        Some(Statement::Let(LetStatement { name, value }))
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_primary_expression()?;

        while self.peek_token.t == TokenType::PLUS
            || self.peek_token.t == TokenType::STAR
            || self.peek_token.t == TokenType::MINUS
            || self.peek_token.t == TokenType::SLASH
        {
            self.next_token(); // Move to operator
            left = self.parse_infix_expression(left)?;
        }

        Some(left)
    }
    

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.literal.clone();

        self.next_token(); // Move to the right-hand side
        let right = self.parse_primary_expression()?;

        Some(Expression::INFEX {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        match self.current_token.t {
            TokenType::INT => {
                let value = self.current_token.literal.parse().ok()?;
                Some(Expression::INT(value))
            }
            TokenType::IDENT => Some(Expression::IDENT(self.current_token.literal.clone())),
            TokenType::TRUE => Some(Expression::BOOLEAN(true)),
            TokenType::FALSE => Some(Expression::BOOLEAN(false)),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
    
        self.next_token(); // Move to the right-hand side
        let right = self.parse_primary_expression()?;
    
        Some(Expression::PREFIX {
            operator,
            right: Box::new(right),
        })
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boolean_expressions() {
        let input = "
        let is_true = true;
        let is_false = false;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 2);

        let expected_identifiers = vec!["is_true", "is_false"];
        let expected_values = vec![true, false];

        for (i, stmt) in program.statements.iter().enumerate() {
            match stmt {
                Statement::Let(let_stmt) => {
                    assert_eq!(let_stmt.name, expected_identifiers[i]);
                    match &let_stmt.value {
                        Expression::BOOLEAN(value) => assert_eq!(*value, expected_values[i]),
                        _ => panic!("Expected BOOLEAN expression"),
                    }
                }
                _ => panic!("Expected LetStatement, got {:?}", stmt),
            }
        }
    }

    #[test]
    fn test_parse_prefix_expressions() {
        let input = "
        let neg = -5;
        let not = !true;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 2);

        let expected_identifiers = vec!["neg", "not"];
        let expected_operators = vec!["-", "!"];
        let expected_values = vec![
            Expression::INT(5),
            Expression::BOOLEAN(true),
        ];

        for (i, stmt) in program.statements.iter().enumerate() {
            match stmt {
                Statement::Let(let_stmt) => {
                    assert_eq!(let_stmt.name, expected_identifiers[i]);
                    match &let_stmt.value {
                        Expression::PREFIX { operator, right } => {
                            assert_eq!(operator, expected_operators[i]);
                            assert_eq!(**right, expected_values[i]);
                        }
                        _ => panic!("Expected PREFIX expression"),
                    }
                }
                _ => panic!("Expected LetStatement, got {:?}", stmt),
            }
        }
    }

    #[test]
    fn test_parse_invalid_prefix_expression() {
        let input = "
        let result = -;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 0, "Expected no valid statements due to invalid prefix expression");
    }

    #[test]
    fn test_parse_multiple_statements_with_errors() {
        let input = "
        let x = 5;
        let y = ;
        let z = 10 + ;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1, "Expected only one valid statement");
        match &program.statements[0] {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.name, "x");
                match &let_stmt.value {
                    Expression::INT(value) => assert_eq!(*value, 5),
                    _ => panic!("Expected INT expression"),
                }
            }
            _ => panic!("Expected LetStatement"),
        }
    }

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
    fn test_parse_expression_without_semicolon() {
        let input = "
        let result = 5 + 10
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 0, "Expected no valid statements due to missing semicolon");
    }

    #[test]
    fn test_parse_invalid_infix_expression() {
        let input = "
        let result = 5 + ;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 0, "Expected no valid statements due to invalid infix expression");
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 0, "Expected no statements for empty input");
    }

    #[test]
    fn test_parse_only_semicolon() {
        let input = ";";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 0, "Expected no statements for input with only a semicolon");
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
