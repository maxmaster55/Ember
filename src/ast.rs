#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    INT(i64),
    INFEX {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    IDENT(String), // For identifiers
    BOOLEAN(bool), // For boolean literals
    PREFIX {
        operator: String,
        right: Box<Expression>,
    }, // For prefix expressions like `!true` or `-5`
}