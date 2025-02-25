#[derive(PartialEq)]
pub enum Operator {
    AddSub(AddSubOperator),
    MultDiv(MultDivOperator),
}

#[derive(PartialEq)]
pub enum AddSubOperator {
    Addition,
    Subtraction,
}

#[derive(PartialEq)]
pub enum MultDivOperator {
    Division,
    Multiplication,
}