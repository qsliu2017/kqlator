use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::{map_res, opt},
    sequence::{pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Scalar(i64),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Minus,
    Multiple,
    Divide,
}

impl From<(Expr, BinaryOp, Expr)> for Expr {
    fn from((lhs, op, rhs): (Expr, BinaryOp, Expr)) -> Self {
        Expr::Binary(lhs.into(), op, rhs.into())
    }
}

fn scalar_expr(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, str::parse::<i64>)(input).map(|(input, scalar)| (input, Expr::Scalar(scalar)))
}

fn parens_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (_, expr, _)) = tuple((one_of("("), binary_expr, one_of(")")))(input)?;
    Ok((input, expr))
}

/// factor := (scalar | parens) ( ('*' | '/') factor)?
fn factor_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (lhs, op_rhs)) = pair(
        alt((scalar_expr, parens_expr)),
        opt(tuple((one_of("*/"), factor_expr))),
    )(input)?;
    if let Some((op, rhs)) = op_rhs {
        let op = match op {
            '*' => BinaryOp::Multiple,
            '/' => BinaryOp::Divide,
            _ => unreachable!(),
        };
        Ok((input, (lhs, op, rhs).into()))
    } else {
        Ok((input, lhs))
    }
}

/// term := factor ( ('+' | '-') term)?
fn term_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (lhs, op_rhs)) =
        tuple((factor_expr, opt(tuple((one_of("+-"), term_expr)))))(input)?;
    if let Some((op, rhs)) = op_rhs {
        let op = match op {
            '+' => BinaryOp::Add,
            '-' => BinaryOp::Minus,
            _ => unreachable!(),
        };
        Ok((input, (lhs, op, rhs).into()))
    } else {
        Ok((input, lhs))
    }
}

pub fn binary_expr(input: &str) -> IResult<&str, Expr> {
    term_expr(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use BinaryOp::*;
    use Expr::*;
    #[test]
    fn test_scalar_expr() {
        for (input, expected) in [("19", Scalar(19)), ("1", Scalar(1))] {
            assert_eq!(scalar_expr(input), Ok(("", expected)), "input: {}", input);
        }
    }
    #[test]
    fn test_factor_expr() {
        for (input, expected) in [
            ("19", Scalar(19)),
            ("1", Scalar(1)),
            ("19*12", (Scalar(19), Multiple, Scalar(12)).into()),
            ("19/12", (Scalar(19), Divide, Scalar(12)).into()),
            (
                "19*12/2",
                (Scalar(19), Multiple, (Scalar(12), Divide, Scalar(2)).into()).into(),
            ),
        ] {
            assert_eq!(factor_expr(input), Ok(("", expected)), "input: {}", input);
        }
    }
    #[test]
    fn test_binary_expr() {
        for (input, expected) in [
            ("19", Scalar(19)),
            ("1", Scalar(1)),
            ("19*12", (Scalar(19), Multiple, Scalar(12)).into()),
            ("19+2", (Scalar(19), Add, Scalar(2)).into()),
            ("1-24", (Scalar(1), Minus, Scalar(24)).into()),
            ("1*28", (Scalar(1), Multiple, Scalar(28)).into()),
            ("16/2", (Scalar(16), Divide, Scalar(2)).into()),
            (
                "1*2+3*4",
                (
                    (Scalar(1), Multiple, Scalar(2)).into(),
                    Add,
                    (Scalar(3), Multiple, Scalar(4)).into(),
                )
                    .into(),
            ),
            (
                "(1+2)*(3-4)",
                (
                    (Scalar(1), Add, Scalar(2)).into(),
                    Multiple,
                    (Scalar(3), Minus, Scalar(4)).into(),
                )
                    .into(),
            ),
        ] {
            assert_eq!(binary_expr(input), Ok(("", expected)), "input: {}", input);
        }
    }
}
