mod expression;
mod number;
mod operator;
mod parser;

use std::env;

use expression::Expression;

fn main() {
    let args: Vec<String> = env::args().collect();
    let equation = &args[1];
    let answer = Expression::evaluate_equation(equation).unwrap();

    println!("{}", answer);
}