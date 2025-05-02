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
        match self.current_token.t {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => {
                // Treat expressions as valid statements
                let expr = self.parse_expression()?;
                Ok(Statement::Expression(ExpressionStatement { expression: expr }))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.next_token();

        let name = match &self.current_token.t {
            TokenType::IDENT => self.current_token.literal.clone(),
            _ => {
                return Err(format!(
                    "Expected identifier, found {:?}",
                    self.current_token
                ));
            }
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
                ));
            }
        }

        Ok(Statement::Let(LetStatement { name, value }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // Skip the "return" token

        let value = self.parse_expression()?;

        self.next_token(); // Expect ';'
        if self.current_token.t != TokenType::SEMICOLON {
            return Err(format!(
                "Expected ';' after return value, found {:?}",
                self.current_token
            ));
        }

        Ok(Statement::Return(ReturnStatement {
            return_value: value,
        }))
    }

    fn parse_condition(&mut self) -> Result<Expression, String> {
        let condition = self.parse_expression()?;

        let ret: Result<Expression, String> = match condition {
            Expression::BOOLEAN(_) => Ok(condition),
            Expression::PREFIX{operator: _, right: _} => Ok(condition),
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
            let stmnt = self.parse_statement()?;
            code.push(stmnt);

            self.next_token(); // Skip the ';'
        }

        println!("token: {:?}", self.current_token);

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

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary_expression()?;

        while self.peek_token.t == TokenType::PLUS
            || self.peek_token.t == TokenType::STAR
            || self.peek_token.t == TokenType::MINUS
            || self.peek_token.t == TokenType::SLASH
            || self.peek_token.t == TokenType::GT
            || self.peek_token.t == TokenType::LT
            || self.peek_token.t == TokenType::EQ
            || self.peek_token.t == TokenType::NEQ
        {
            self.next_token(); // Move to operator
            left = self.parse_infix_expression(left)?;
        }

        Ok(left)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        self.next_token(); // Move to the right-hand side
        
        let operator = self.current_token.literal.clone();
        
        println!("OP ==> {}", operator);
        
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
            TokenType::IF => {
                return self.parse_if_expression();
            }
            TokenType::INT => {
                let value = self.current_token.literal.parse().map_err(|_| {
                    format!("Invalid integer literal: {}", self.current_token.literal)
                })?;
                return Ok(Expression::INT(value));
            }
            TokenType::TRUE => Ok(Expression::BOOLEAN(true)),
            TokenType::FALSE => Ok(Expression::BOOLEAN(false)),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::IDENT => self.parse_infix_expression(Expression::IDENT(self.current_token.literal.clone())),
            _ => Err(format!(
                "Unexpected token {:?} in primary expression",
                self.current_token
            )),
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

