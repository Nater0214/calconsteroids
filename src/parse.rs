use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

use crate::expression::Expression;

/// An expression parser
#[derive(Parser)]
#[grammar = "latex_expression.pest"]
pub struct LatexExpressionParser;

lazy_static::lazy_static! {
    static ref PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(plus, Left) | Op::infix(minus, Left))
            .op(Op::infix(cdot, Left) | Op::infix(asterisk, Left) | Op::infix(slash, Left))
            .op(Op::prefix(negate))
            .op(Op::postfix(factorial))
    };
}

/// Parse a LaTeX math expression
#[inline]
pub fn parse_latex(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    LatexExpressionParser::parse(Rule::expression, input)
}

/// Parse pairs
pub fn parse_pairs(pairs: Pairs<Rule>) -> Expression {
    PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expression::Value(
                primary
                    .as_str()
                    .parse()
                    .unwrap(),
            ),
            Rule::variable => Expression::Variable(
                primary
                    .as_str()
                    .to_string(),
            ),
            Rule::implicit_multiplication => {
                let mut inner = primary
                    .into_inner()
                    .rev();
                let mut expression = parse_pairs(Pairs::single(
                    inner
                        .next()
                        .unwrap(),
                ));
                for pair in inner {
                    expression = Expression::Multiplication(Box::new(parse_pairs(Pairs::single(pair))), Box::new(expression));
                }
                expression
            }
            Rule::paren_expression => parse_pairs(primary.into_inner()),
            Rule::expression => parse_pairs(primary.into_inner()),
            rule => unreachable!("Unexpected rule: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let lhs: Box<Expression> = Box::new(lhs);
            let rhs = Box::new(rhs);
            match op.as_rule() {
                Rule::plus => Expression::Addition(lhs, rhs),
                Rule::minus => Expression::Subtraction(lhs, rhs),
                Rule::cdot => Expression::Multiplication(lhs, rhs),
                Rule::asterisk => Expression::Multiplication(lhs, rhs),
                Rule::slash => Expression::Division(lhs, rhs),
                rule => unreachable!("Unexpected rule: {:?}", rule),
            }
        })
        .map_prefix(|op, rhs| {
            let rhs = Box::new(rhs);
            match op.as_rule() {
                Rule::negate => Expression::Negation(rhs),
                rule => unreachable!("Unexpected rule: {:?}", rule),
            }
        })
        .map_postfix(|lhs, op| {
            let lhs = Box::new(lhs);
            match op.as_rule() {
                Rule::factorial => Expression::Factorial(lhs),
                rule => unreachable!("Unexpected rule: {:?}", rule),
            }
        })
        .parse(pairs)
}
