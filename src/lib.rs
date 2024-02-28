use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Scalar(i64),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i64 {
        match self {
            Expr::Scalar(val) => *val,
            Expr::Binary(lhs, op, rhs) => match op {
                BinaryOp::Add => lhs.eval() + rhs.eval(),
                BinaryOp::Minus => lhs.eval() - rhs.eval(),
                BinaryOp::Multiple => lhs.eval() * rhs.eval(),
                BinaryOp::Divide => lhs.eval() / rhs.eval(),
            },
        }
    }
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
    let (input, scalar) = digit1(input)?;
    Ok((input, Expr::Scalar(scalar.parse().unwrap())))
}

fn parens_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (_, expr, _)) = tuple((one_of("("), binary_expr, one_of(")")))(input)?;
    Ok((input, expr))
}

/// factor := scalar | parens
fn factor_expr(input: &str) -> IResult<&str, Expr> {
    let (input, expr) = alt((parens_expr, scalar_expr))(input)?;
    Ok((input, expr))
}

/// term := factor ( ('*' | '/') factor)*
fn term_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (first, rest)) =
        tuple((factor_expr, many0(tuple((one_of("*/"), factor_expr)))))(input)?;
    let expr = rest.into_iter().fold(first, |expr, (op, factor)| {
        let op = match op {
            '*' => BinaryOp::Multiple,
            '/' => BinaryOp::Divide,
            _ => unreachable!(),
        };
        (expr, op, factor).into()
    });
    Ok((input, expr))
}

/// binary := term ( ('+' | '-') term)*
pub fn binary_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (first, rest)) =
        tuple((term_expr, many0(tuple((one_of("+-"), term_expr)))))(input)?;
    let expr = rest.into_iter().fold(first, |expr, (op, term)| {
        let op = match op {
            '+' => BinaryOp::Add,
            '-' => BinaryOp::Minus,
            _ => unreachable!(),
        };
        (expr, op, term).into()
    });
    Ok((input, expr))
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
    fn test_term_expr() {
        for (input, expected) in [
            ("19", Scalar(19)),
            ("1", Scalar(1)),
            ("19*12", (Scalar(19), Multiple, Scalar(12)).into()),
            ("19/12", (Scalar(19), Divide, Scalar(12)).into()),
            (
                "19*12/2",
                ((Scalar(19), Multiple, Scalar(12)).into(), Divide, Scalar(2)).into(),
            ),
        ] {
            assert_eq!(term_expr(input), Ok(("", expected)), "input: {}", input);
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
            (
                "1*2/3+4/5*6",
                (
                    ((Scalar(1), Multiple, Scalar(2)).into(), Divide, Scalar(3)).into(),
                    Add,
                    ((Scalar(4), Divide, Scalar(5)).into(), Multiple, Scalar(6)).into(),
                )
                    .into(),
            ),
        ] {
            assert_eq!(binary_expr(input), Ok(("", expected)), "input: {}", input);
        }
    }
}
