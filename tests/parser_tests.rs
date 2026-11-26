use tinyexpr::{eval, parse, BinaryOp, Expr};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

#[test]
fn arithmetic_precedence() {
    close(eval("1 + 2 * 3").unwrap(), 7.0);
    close(eval("(1 + 2) * 3").unwrap(), 9.0);
    close(eval("8 / 4 * 2").unwrap(), 4.0);
    close(eval("10 - 3 - 2").unwrap(), 5.0);
}

#[test]
fn exponent_is_right_associative() {
    close(eval("2 ^ 3 ^ 2").unwrap(), 512.0);
}

#[test]
fn unary_minus_and_exponent_precedence() {
    close(eval("-2 ^ 2").unwrap(), -4.0);
    close(eval("(-2) ^ 2").unwrap(), 4.0);
    close(eval("2 ^ -3").unwrap(), 0.125);
    close(eval("--2 ^ 2").unwrap(), 4.0);
}

#[test]
fn decimals_and_whitespace() {
    close(eval(" .5 + 1.25 ").unwrap(), 1.75);
    close(eval("3.").unwrap(), 3.0);
}

#[test]
fn builds_expected_ast_shape() {
    let expr = parse("2 ^ 3 ^ 2").unwrap();
    match expr {
        Expr::Binary { op: BinaryOp::Power, right, .. } => {
            assert!(matches!(*right, Expr::Binary { op: BinaryOp::Power, .. }));
        }
        other => panic!("unexpected AST: {other:?}"),
    }
}

#[test]
fn rejects_invalid_expressions() {
    assert!(parse("").is_err());
    assert!(parse("1 +").is_err());
    assert!(parse("(1 + 2").is_err());
    assert!(parse("1 2").is_err());
    assert!(parse("1 @ 2").is_err());
}

#[test]
fn division_by_zero_is_an_evaluation_error() {
    assert!(eval("1 / 0").is_err());
}
