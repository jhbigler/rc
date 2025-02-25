use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, tuple},
    IResult
};

use crate::{
    expression::Expression,
    operator::{AddSubOperator, MultDivOperator, Operator},
    number::Number,
};

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, first_term) = parse_term(input)?;
    let (input, ops_term) = many0(tuple((
        multispace0,
        parse_add_sub_operator,
        multispace0,
        parse_term
    )))(input)?;

    let result = ops_term.into_iter().fold(first_term, |acc, (_1, op, _2, term)| {
        Expression::Operation(Box::new(acc), Operator::AddSub(op), Box::new(term))
    });

    Ok((input, result))
}

fn parse_add_sub_operator(input: &str) -> IResult<&str, AddSubOperator> {
    let (input, op) = one_of("+-")(input)?;
    let operator = match op {
        '-' => AddSubOperator::Subtraction,
        '+' => AddSubOperator::Addition,
        _   => unreachable!(),
    };

    Ok((input, operator))
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, _) = multispace0(input)?;
    let (input, first_factor) = parse_factor(input)?;
    let (input, ops_factor) = many0(tuple((
        multispace0,
        parse_mult_div_operator,
        multispace0,
        parse_factor,
        multispace0
    )))(input)?;

    let result = ops_factor.into_iter().fold(first_factor, |acc, (_1, op,_2, factor, _3)|{
        Expression::Operation(Box::new(acc), Operator::MultDiv(op), Box::new(factor))
    });

    Ok((input, result))
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_number, Expression::Number),
        delimited(char('('), parse_expression, char(')')),
        ))(input)
}

fn parse_mult_div_operator(input: &str) -> IResult<&str, MultDivOperator> {
    let (input, op) = one_of("*/")(input)?;
    let operator = match op {
        '*' => MultDivOperator::Multiplication,
        '/' => MultDivOperator::Division,
        _   => unreachable!(),
    };

    Ok((input, operator))
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    // Parse an optional sign
    let (input, sign) = opt(one_of("+-"))(input)?;

    // Parse a float: digits, a decimal point, and more digits
    let float_parser = map(
        recognize(tuple((digit1, one_of("."), digit1))),
        |s: &str| Number::Float(s.parse::<f64>().unwrap()),
    );

    // Parse an integer: digits only
    let int_parser = map(
        digit1,
        |s: &str| Number::Integer(s.parse::<i64>().unwrap()),
    );

    // Try to parse a float first, then fall back to an integer
    let (input, mut number) = alt((float_parser, int_parser))(input)?;

    // Apply the sign if it exists
    if let Some(sign_char) = sign {
        match number {
            Number::Integer(i) => number = Number::Integer(if sign_char == '-' { -i } else { i }),
            Number::Float(f) => number = Number::Float(if sign_char == '-' { -f } else { f }),
        }
    }

    Ok((input, number))
}