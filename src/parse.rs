use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

use crate::{
    expression::{
        AdditionExpression, DivisionExpression, Expression, MultiplicationExpression,
        SubtractionExpression, ValueExpression,
    },
    value::RationalValue,
};

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
            .op(Op::infix(carat, Left))
    };
}

/// Parse a LaTeX math expression
#[inline]
pub fn parse_latex(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    LatexExpressionParser::parse(Rule::expression, input)
}

/// Parse pairs
pub fn parse_pairs(pairs: Pairs<Rule>) -> Box<dyn Expression> {
    PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Box::new(ValueExpression::new(Box::new(
                primary.as_str().parse::<RationalValue>().unwrap(),
            ))),
            Rule::implicit_multiplication => {
                let mut inner = primary.into_inner().rev();
                let mut expression = parse_pairs(Pairs::single(inner.next().unwrap()));
                for pair in inner {
                    expression = Box::new(MultiplicationExpression::new(
                        expression,
                        parse_pairs(Pairs::single(pair)),
                    ));
                }
                expression
            }
            Rule::paren_expression => parse_pairs(primary.into_inner()),
            Rule::expression => parse_pairs(primary.into_inner()),
            rule => unreachable!("Unexpected rule: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::plus => Box::new(AdditionExpression::new(lhs, rhs)),
            Rule::minus => Box::new(SubtractionExpression::new(lhs, rhs)),
            Rule::asterisk => Box::new(MultiplicationExpression::new(lhs, rhs)),
            Rule::cdot => Box::new(MultiplicationExpression::new(lhs, rhs)),
            Rule::slash => Box::new(DivisionExpression::new(lhs, rhs)),
            rule => unreachable!("Unexpected rule: {:?}", rule),
        })
        .map_prefix(|op, rhs| {
            let rhs = Box::new(rhs);
            match op.as_rule() {
                rule => unreachable!("Unexpected rule: {:?}", rule),
            }
        })
        .map_postfix(|lhs, op| {
            let lhs = Box::new(lhs);
            match op.as_rule() {
                rule => unreachable!("Unexpected rule: {:?}", rule),
            }
        })
        .parse(pairs)
}
