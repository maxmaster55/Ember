use crate::ast::{Expression, Program, Statement};
use crate::object::{Object, ObjectType};

pub fn eval(Statement: Statement) -> impl Object {
    match Statement {
        Statement::Expression(expr) => eval_expression(expr.expression),
        _ => panic!("Unknown statement type"),
    }
}

fn eval_expression(expr: Expression) -> ObjectType {
    match expr {
        Expression::INT(val) => ObjectType::Number(val),
        Expression::BOOLEAN(val) => bool_to_bool_object(val),
        Expression::INFEX {
            left,
            operator,
            right,
        } => eval_infex_expression(*left, operator, *right),
        Expression::PREFIX { operator, right } => eval_prefex_expression(operator, *right),
        _ => panic!("Unknown expression type"),
    }
}

fn eval_prefex_expression(operator: String, right: Expression) -> ObjectType {
    let right = eval_expression(right);

    match operator.as_str() {
        "-" => match right {
            ObjectType::Number(val) => ObjectType::Number(-val),
            _ => panic!("Unsupported type for '-' operator"),
        },
        "!" => match right {
            ObjectType::Boolean(val) => ObjectType::Boolean(!val),
            _ => panic!("Unsupported type for '!' operator"),
        },
        _ => panic!("Unknown prefix operator"),
    }
}

fn eval_infex_expression(left: Expression, operator: String, right: Expression) -> ObjectType {
    let left = eval_expression(left);
    let right = eval_expression(right);

    match operator.as_str() {
        "+" => match (left, right) {
            (ObjectType::Number(l), ObjectType::Number(r)) => ObjectType::Number(l + r),
            _ => panic!("Unsupported types for '+' operator"),
        },
        "-" => match (left, right) {
            (ObjectType::Number(l), ObjectType::Number(r)) => ObjectType::Number(l - r),
            _ => panic!("Unsupported types for '-' operator"),
        },
        "*" => match (left, right) {
            (ObjectType::Number(l), ObjectType::Number(r)) => ObjectType::Number(l * r),
            _ => panic!("Unsupported types for '*' operator"),
        },
        "/" => match (left, right) {
            (ObjectType::Number(l), ObjectType::Number(r)) => ObjectType::Number(l / r),
            _ => panic!("Unsupported types for '/' operator"),
        },
        _ => panic!("Unknown operator"),
    }
}

fn bool_to_bool_object(val: bool) -> ObjectType {
    if val {
        ObjectType::Boolean(true)
    } else {
        ObjectType::Boolean(false)
    }
}
