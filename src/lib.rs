pub mod ast;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod token;

pub use ast::{BinaryOp, Expr, UnaryOp};
pub use error::{EvalError, LexError, ParseError};
pub use evaluator::{evaluate, Environment};
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::{Span, Token, TokenKind};

/// Parse an input string into an AST.
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let tokens = Lexer::new(input).tokenize()?;
    Parser::new(tokens).parse()
}

/// Parse and evaluate an arithmetic expression.
pub fn eval(input: &str) -> Result<f64, MathError> {
    let expr = parse(input).map_err(MathError::Parse)?;
    evaluate(&expr, &Environment::new()).map_err(MathError::Eval)
}

#[derive(Debug)]
pub enum MathError {
    Parse(ParseError),
    Eval(EvalError),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "{e}"),
            Self::Eval(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for MathError {}
