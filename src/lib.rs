use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, sequence::tuple, IResult,
};

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Minus,
    Multiple,
    Divide,
}

fn binary_op(input: &str) -> IResult<&str, BinaryOp> {
    alt((tag("+"), tag("-"), tag("*"), tag("/")))(input).map(|(input, op)| {
        (
            input,
            match op {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Minus,
                "*" => BinaryOp::Multiple,
                "/" => BinaryOp::Divide,
                _ => unreachable!(),
            },
        )
    })
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr(i64, BinaryOp, i64);

fn binary_expr(input: &str) -> IResult<&str, BinaryExpr> {
    let (input, (lhs, op, rhs)) = tuple((digit1, binary_op, digit1))(input)?;
    Ok((
        input,
        BinaryExpr(lhs.parse().unwrap(), op, rhs.parse().unwrap()),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary_expr() {
        let input = "19+2";
        let result = binary_expr(input);
        assert_eq!(result, Ok(("", BinaryExpr(19, BinaryOp::Add, 2))));
    }
}
