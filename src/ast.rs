
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

#[derive(Debug)]
pub enum Expression {
    INT(i64),
    INFEX{
        left: Box<Expression>,
        operator:String,
        right:Box<Expression>
    },

}