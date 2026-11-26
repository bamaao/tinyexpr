use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::error::EvalError;
use std::collections::HashMap;

/// Reserved for the next extension: variables can be added without changing
/// the lexer/parser architecture (identifier tokens will be added then).
pub type Environment = HashMap<String, f64>;

pub fn evaluate(expr: &Expr, _env: &Environment) -> Result<f64, EvalError> {
    let value = match expr {
        Expr::Number(value) => *value,
        Expr::Unary { op, expr } => {
            let value = evaluate(expr, _env)?;
            match op {
                UnaryOp::Plus => value,
                UnaryOp::Negate => -value,
            }
        }
        Expr::Binary { op, left, right } => {
            let lhs = evaluate(left, _env)?;
            let rhs = evaluate(right, _env)?;
            match op {
                BinaryOp::Add => lhs + rhs,
                BinaryOp::Subtract => lhs - rhs,
                BinaryOp::Multiply => lhs * rhs,
                BinaryOp::Divide => {
                    if rhs == 0.0 {
                        return Err(EvalError::DivisionByZero);
                    }
                    lhs / rhs
                }
                BinaryOp::Power => lhs.powf(rhs),
            }
        }
    };

    if value.is_finite() {
        Ok(value)
    } else {
        Err(EvalError::NonFiniteResult)
    }
}
