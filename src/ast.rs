#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}


#[derive(Debug, PartialEq)]
pub enum Expression {
    INT(i64),
    INFEX {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    IDENT(String),
    BOOLEAN(bool),
    PREFIX {
        operator: String,
        right: Box<Expression>,
    },
    IF {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>
    },
    FUNCTION {
        parameters: Vec<String>,
        body: Box<Statement>,
    },
    CALL {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

