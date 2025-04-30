use crate::{ast::{Expression, IfStatement, LetStatement, Program, Statement}, lexer::Lexer, token::{Token, TokenType}};

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

pub fn parse_program(&mut self) -> Result<Program, String> {
    let mut statements = vec![];

    while self.current_token.t != TokenType::EOF {
        // Skip semicolons that appear between statements
        if self.current_token.t == TokenType::SEMICOLON {
            self.next_token();
            continue;
        }

        match self.parse_statement() {
            Ok(stmt) => statements.push(stmt),
            Err(err) => {
                eprintln!("Error parsing statement: {}", err);
                self.next_token(); // Skip the invalid token
            }
        }
    }

    Ok(Program { statements })
}


    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token.t {
            TokenType::LET => self.parse_let_statement(),
            TokenType::IF => self.parse_if_statement(),
            _ => Err(format!(
                "Unexpected token {:?} at statement level",
                self.current_token
            )),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.next_token();

        let name = match &self.current_token.t {
            TokenType::IDENT => self.current_token.literal.clone(),
            _ => return Err(format!("Expected identifier, found {:?}", self.current_token)),
        };

        self.next_token(); // Expect '='
        if self.current_token.t != TokenType::ASSIGN {
            return Err(format!(
                "Expected '=', found {:?} after identifier '{}'",
                self.current_token, name
            ));
        }

        self.next_token();

        let value = self.parse_expression()?; // Parse the expression

        self.next_token();
        match self.current_token.t {
            TokenType::SEMICOLON => {}
            _ => {
                return Err(format!(
                    "Expected ';' after expression, found {:?}",
                    self.current_token
                ))
            }
        }

        Ok(Statement::Let(LetStatement { name, value }))
    }

    fn parse_condition(&mut self) -> Result<Expression, String> {
        let condition = self.parse_expression()?;
        
        let ret = match condition {
            Expression::BOOLEAN(_) => Ok(condition),
            Expression::INFEX { ref operator, .. } if operator == "==" || operator == "!=" || operator == ">" || operator == "<" => Ok(condition),
            _ => Err(format!("Invalid condition: {:?}. Expected a boolean expression or comparison.", condition)),
        };
        self.next_token();
        return ret;
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // Skip the "if" token
    
        // Parse the condition
        let cond = self.parse_condition()?;
    
        if self.current_token.t != TokenType::LBRACE {
            return Err("Expected '{' after an IF condition".to_string());
        }
        self.next_token(); // skip the '{'

        let mut code= vec![];

        while self.current_token.t != TokenType::RBRACE {

            let stmnt = self.parse_statement()?;
            code.push(stmnt);
    
            self.next_token(); // Skip the ';'
        }


        println!("token: {:?}", self.current_token);


        if self.current_token.t != TokenType::RBRACE {
            return Err("Expected '}' after an IF condition".to_string());
        }
        self.next_token(); // Skip the '}'

        return Ok(Statement::If(IfStatement { cond, code }));
    }



    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary_expression()?;

        while self.peek_token.t == TokenType::PLUS
            || self.peek_token.t == TokenType::STAR
            || self.peek_token.t == TokenType::MINUS
            || self.peek_token.t == TokenType::SLASH
        {
            self.next_token(); // Move to operator
            left = self.parse_infix_expression(left)?;
        }

        Ok(left)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let operator = self.current_token.literal.clone();

        self.next_token(); // Move to the right-hand side
        let right = self
            .parse_primary_expression()
            .map_err(|err| format!("Error parsing right-hand side of infix expression: {}", err))?;

        Ok(Expression::INFEX {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, String> {
        match self.current_token.t {
            TokenType::INT => {
                let value = self
                    .current_token
                    .literal
                    .parse()
                    .map_err(|_| format!("Invalid integer literal: {}", self.current_token.literal))?;
                Ok(Expression::INT(value))
            }
            TokenType::IDENT => Ok(Expression::IDENT(self.current_token.literal.clone())),
            TokenType::TRUE => Ok(Expression::BOOLEAN(true)),
            TokenType::FALSE => Ok(Expression::BOOLEAN(false)),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            _ => Err(format!(
                "Unexpected token {:?} in primary expression",
                self.current_token
            )),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        let operator = self.current_token.literal.clone();

        self.next_token(); // Move to the right-hand side
        let right = self
            .parse_primary_expression()
            .map_err(|err| format!("Error parsing right-hand side of prefix expression: {}", err))?;

        Ok(Expression::PREFIX {
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

        let program = parser.parse_program().unwrap();
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

        let program = parser.parse_program().unwrap();
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

        let program = parser.parse_program().unwrap_or_else(|_| Program { statements: vec![] });
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

        let program = parser.parse_program().unwrap();
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

        let program = parser.parse_program().unwrap();
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

        let program = parser.parse_program().unwrap();
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

        let program = parser.parse_program().unwrap_or_else(|_| Program { statements: vec![] });
        assert_eq!(program.statements.len(), 0, "Expected no valid statements due to missing semicolon");
    }

    #[test]
    fn test_parse_invalid_infix_expression() {
        let input = "
        let result = 5 + ;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap_or_else(|_| Program { statements: vec![] });
        assert_eq!(program.statements.len(), 0, "Expected no valid statements due to invalid infix expression");
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 0, "Expected no statements for empty input");
    }

    #[test]
    fn test_parse_only_semicolon() {
        let input = ";";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 0, "Expected no statements for input with only a semicolon");
    }

    #[test]
    fn test_invalid_let_statement() {
        let input = "
        let x 5;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap_or_else(|_| Program { statements: vec![] });
        println!("{:?}", program);
    }

    #[test]
    fn test_parse_if_statement_with_complex_expression() {
        let input = "
        if true {
            let x = 5 + 5 + 5 + 5 + 5;
        }
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::If(if_stmt) => {
                assert_eq!(if_stmt.cond, Expression::BOOLEAN(true));
                assert_eq!(if_stmt.code.len(), 1);
                match &if_stmt.code[0] {
                    Statement::Let(let_stmt) => {
                        assert_eq!(let_stmt.name, "x");
                        match &let_stmt.value {
                            Expression::INFEX { left, operator, right } => {
                                assert_eq!(operator, "+");
                                match **left {
                                    Expression::INFEX { .. } => {} // Nested infix expressions
                                    _ => panic!("Expected nested INFEX"),
                                }
                                match **right {
                                    Expression::INT(value) => assert_eq!(value, 5),
                                    _ => panic!("Expected INT on the right"),
                                }
                            }
                            _ => panic!("Expected INFEX expression"),
                        }
                    }
                    _ => panic!("Expected LetStatement"),
                }
            }
            _ => panic!("Expected IfStatement"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "
        if true {
            let x = 5;
        }
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::If(if_stmt) => {
                assert_eq!(if_stmt.cond, Expression::BOOLEAN(true));
                assert_eq!(if_stmt.code.len(), 1);
                match &if_stmt.code[0] {
                    Statement::Let(let_stmt) => {
                        assert_eq!(let_stmt.name, "x");
                        assert_eq!(let_stmt.value, Expression::INT(5));
                    }
                    _ => panic!("Expected LetStatement"),
                }
            }
            _ => panic!("Expected IfStatement"),
        }
    }

    // add test for if statment with multiple statements
    #[test]
    fn test_parse_if_statement_with_multiple_statements() {
        let input = "
        if true {
            let x = 5;
            let y = 10;
        }
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::If(if_stmt) => {
                assert_eq!(if_stmt.cond, Expression::BOOLEAN(true));
                assert_eq!(if_stmt.code.len(), 2);
                match &if_stmt.code[0] {
                    Statement::Let(let_stmt) => {
                        assert_eq!(let_stmt.name, "x");
                        assert_eq!(let_stmt.value, Expression::INT(5));
                    }
                    _ => panic!("Expected LetStatement"),
                }
                match &if_stmt.code[1] {
                    Statement::Let(let_stmt) => {
                        assert_eq!(let_stmt.name, "y");
                        assert_eq!(let_stmt.value, Expression::INT(10));
                    }
                    _ => panic!("Expected LetStatement"),
                }
            }
            _ => panic!("Expected IfStatement"),
        }
    }

}
