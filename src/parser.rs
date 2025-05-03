use crate::ast::{
    Expression, ExpressionStatement, LetStatement, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenType;

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
            peek_token,
        };
    }

    fn next_token(&mut self) {
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
                    while self.current_token.t != TokenType::EOF {
                        self.next_token(); // Skip to end
                    }
                }
            }
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        let stmnt = match self.current_token.t {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => {
                // Treat expressions as valid statements
                let expr = self.parse_expression()?;
                Ok(Statement::Expression(ExpressionStatement {
                    expression: expr,
                }))
            }
        };

        if self.peek_token.t == TokenType::SEMICOLON {
            self.next_token();
        }

        return stmnt;
    }


    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        // Move to the identifier
        self.next_token();
    
        // Expect identifier
        let identifier = match &self.current_token.t {
            TokenType::IDENT => self.current_token.literal.clone(),
            _ => {
                return Err(format!(
                    "Expected identifier, found {:?}",
                    self.current_token
                ));
            }
        };
    
        // Expect '='
        self.next_token();
        if self.current_token.t != TokenType::ASSIGN {
            return Err(format!(
                "Expected '=', found {:?}",
                self.current_token
            ));
        }
    
        // Move to the start of the expression
        self.next_token();
    
        // Parse the expression on the right-hand side of the assignment
        let value = self.parse_expression()?;
    
        // Construct the let statement
        Ok(Statement::Let( LetStatement{name: identifier,value} ))
    }
    


    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // Skip the "return" token

        let value = self.parse_expression()?;

        Ok(Statement::Return(ReturnStatement {
            return_value: value,
        }))
    }

    fn parse_condition(&mut self) -> Result<Expression, String> {
        let condition = self.parse_expression()?;

        let ret: Result<Expression, String> = match condition {
            Expression::BOOLEAN(_) => Ok(condition),
            Expression::PREFIX {
                operator: _,
                right: _,
            } => Ok(condition),
            Expression::INFEX { ref operator, .. }
                if operator == "==" || operator == "!=" || operator == ">" || operator == "<" =>
            {
                Ok(condition)
            }
            _ => Err(format!(
                "Invalid condition: {:?}. Expected a boolean expression or comparison.",
                condition
            )),
        };
        self.next_token();
        return ret;
    }

    fn parse_if_expression(&mut self) -> Result<Expression, String> {
        self.next_token(); // Skip the "if" token
        
        // Parse the condition
        let cond = self.parse_condition()?;


        if self.current_token.t != TokenType::LBRACE {
            return Err("Expected '{' after an IF condition".to_string());
        }
        self.next_token(); // skip the '{'

        let mut code = vec![];

        while self.current_token.t != TokenType::RBRACE {
            // Skip semicolons that appear between statements
            if self.current_token.t == TokenType::SEMICOLON {
                self.next_token();
                continue;
            }
            let stmnt = self.parse_statement()?;
            code.push(stmnt);
        }

        // Check for the closing brace
        if self.current_token.t != TokenType::RBRACE {
            return Err("Expected '}' after an IF condition".to_string());
        }
        self.next_token(); // Skip the '}'

        // Check for optional else block
        if self.current_token.t == TokenType::ELSE {
            self.next_token(); // Skip the "else" token
            if self.current_token.t != TokenType::LBRACE {
                return Err("Expected '{' after 'else'".to_string());
            }
            self.next_token(); // Skip the '{'
            let mut else_code = vec![];

            while self.current_token.t != TokenType::RBRACE {
                let stmnt = self.parse_statement()?;
                else_code.push(stmnt);
                self.next_token(); // Skip the ';'
            }

            if self.current_token.t != TokenType::RBRACE {
                return Err("Expected '}' after 'else' block".to_string());
            }
            self.next_token(); // Skip the '}'

            return Ok(Expression::IF {
                condition: Box::new(cond),
                consequence: code,
                alternative: Some(else_code),
            });
        }

        return Ok(Expression::IF {
            condition: Box::new(cond),
            consequence: code,
            alternative: None,
        });
    }

    fn is_operator(tok: &Token) -> bool {
        return tok.t == TokenType::PLUS
            || tok.t == TokenType::STAR
            || tok.t == TokenType::MINUS
            || tok.t == TokenType::SLASH
            || tok.t == TokenType::GT
            || tok.t == TokenType::LT
            || tok.t == TokenType::EQ
            || tok.t == TokenType::NEQ;
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary_expression()?;


        if self.peek_token.t == TokenType::RPAREN {
            self.next_token();
        }

        // Continue parsing infix expressions while we see operators
        // and we haven't hit a closing parenthesis
        while Parser::is_operator(&self.peek_token) {
            left = self.parse_infix_expression(left)?;
        }

        Ok(left)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        
        self.next_token();
        // Check if the next is a )

        let operator = self.current_token.literal.clone();

        self.next_token();
    
        let right = self
            .parse_expression()
            .map_err(|err| format!("Error parsing right-hand side of infix expression: {}", err))?;
    
        Ok(Expression::INFEX {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    

    fn parse_primary_expression(&mut self) -> Result<Expression, String> {


        match self.current_token.t {
            TokenType::IF => self.parse_if_expression(),
            TokenType::INT => self.parse_integer_literal(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean_literal(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::IDENT => self.parse_identifier_expression(),
            _ => Err(format!(
                "Unexpected token {:?} in primary expression",
                self.current_token
            )),
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
        self.next_token(); // Skip '('
    
        let expr = self.parse_expression()?; // Parse full expression like 5 + 5
    
        // Now advance once to move to the next token, which should be ')'
        self.next_token();
        
        Ok(expr)
    }
    

    fn parse_integer_literal(&mut self) -> Result<Expression, String> {
        let value = self
            .current_token
            .literal
            .parse()
            .map_err(|_| format!("Invalid integer literal: {}", self.current_token.literal))?;

        Ok(Expression::INT(value))
    }

    fn parse_boolean_literal(&mut self) -> Result<Expression, String> {
        match self.current_token.t {
            TokenType::TRUE => Ok(Expression::BOOLEAN(true)),
            TokenType::FALSE => Ok(Expression::BOOLEAN(false)),
            _ => Err(format!(
                "Unexpected token {:?} in boolean literal",
                self.current_token
            )),
        }
    }

    fn parse_identifier_expression(&mut self) -> Result<Expression, String> {
        if Parser::is_operator(&self.peek_token) {
            self.parse_infix_expression(Expression::IDENT(self.current_token.literal.clone()))
        } else {
            Ok(Expression::IDENT(self.current_token.literal.clone()))
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        let operator = self.current_token.literal.clone();

        self.next_token(); // Move to the right-hand side
        let right = self.parse_primary_expression().map_err(|err| {
            format!(
                "Error parsing right-hand side of prefix expression: {}",
                err
            )
        })?;

        Ok(Expression::PREFIX {
            operator,
            right: Box::new(right),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_parser(input: String) -> Parser {
        let lexer = Lexer::new(input);
        Parser::new(lexer)
    }

    #[test]
    fn test_parse_let_statement() {
        let input = "let x = 5;".to_string();
        let mut parser = setup_parser(input);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let Statement::Let(let_stmt) = &program.statements[0] {
            assert_eq!(let_stmt.name, "x");
            if let Expression::INT(value) = let_stmt.value {
                assert_eq!(value, 5);
            } else {
                panic!("Expected integer literal in let statement");
            }
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_parse_return_statement() {
        let input = "ret 10;".to_string();
        let mut parser = setup_parser(input);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let Statement::Return(return_stmt) = &program.statements[0] {
            if let Expression::INT(value) = return_stmt.return_value {
                assert_eq!(value, 10);
            } else {
                panic!("Expected integer literal in return statement");
            }
        } else {
            panic!("Expected return statement");
        }
    }

    #[test]
    fn test_parse_if_expression() {
        let input = "if x < 10 { let y = 5; } else { ret 20; }".to_string();
        let mut parser = setup_parser(input);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Expression::IF { condition, consequence, alternative } = &expr_stmt.expression {
                if let Expression::INFEX { ref operator, .. } = **condition {
                    assert_eq!(operator, "<");
                } else {
                    panic!("Expected infix expression in if condition");
                }

                assert_eq!(consequence.len(), 1);
                if let Statement::Let(let_stmt) = &consequence[0] {
                    assert_eq!(let_stmt.name, "y");
                } else {
                    panic!("Expected let statement in if consequence");
                }

                assert!(alternative.is_some());
                let alternative = alternative.as_ref().unwrap();
                assert_eq!(alternative.len(), 1);
                if let Statement::Return(return_stmt) = &alternative[0] {
                    if let Expression::INT(value) = return_stmt.return_value {
                        assert_eq!(value, 20);
                    } else {
                        panic!("Expected integer literal in return statement");
                    }
                } else {
                    panic!("Expected return statement in if alternative");
                }
            } else {
                panic!("Expected if expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_infix_expression() {
        let input = "5 + 10 * 2;".to_string();
        let mut parser = setup_parser(input);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Expression::INFEX { operator, .. } = &expr_stmt.expression {
                assert_eq!(operator, "+");
            } else {
                panic!("Expected infix expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_boolean_literal() {
        let input = "true;".to_string();
        let mut parser = setup_parser(input);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Expression::BOOLEAN(value) = &expr_stmt.expression {
                assert_eq!(*value, true);
            } else {
                panic!("Expected boolean literal");
            }
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_grouped_expression() {
        let input = "(5 + 5) * 2;".to_string();
        let mut parser = setup_parser(input);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Expression::INFEX { operator, .. } = &expr_stmt.expression {
                assert_eq!(operator, "*");
            } else {
                panic!("Expected infix expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }
}

