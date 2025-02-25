/*
    Expressions are either just a single number or an Operation
*/

use crate::{
    number::Number,
    operator::{Operator, AddSubOperator, MultDivOperator},
    parser::parse_expression,
};

pub enum Expression {
    Number(Number),
    Operation(Box<Expression>, Operator, Box<Expression>),
}

impl Expression {
    pub fn evaluate_equation(equation: &str) -> Result<Number, &str> {
        let (_, num) = parse_expression(equation).unwrap();
        Ok(num.evaluate())
    }

    fn evaluate(&self) -> Number {
        match self {
            Expression::Number(val) => *val,
            Expression::Operation(lhs, op, rhs ) => {
                let lhs_val = lhs.as_ref().evaluate();
                let rhs_val = rhs.as_ref().evaluate();
                match op {
                    Operator::AddSub(addsub) => match addsub {
                        AddSubOperator::Addition => lhs_val + rhs_val,
                        AddSubOperator::Subtraction => lhs_val - rhs_val,
                    },
                    Operator::MultDiv(md) => match md {
                        MultDivOperator::Multiplication => lhs_val * rhs_val,
                        MultDivOperator::Division => lhs_val / rhs_val,
                    }
                }
            }
        }
    }
}