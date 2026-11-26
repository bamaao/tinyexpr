use crate::token::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "词法错误（{}..{}）：{}", self.span.start, self.span.end, self.message)
    }
}
impl std::error::Error for LexError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "语法错误（{}..{}）：{}", self.span.start, self.span.end, self.message)
    }
}
impl std::error::Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        Self {
            message: err.message,
            span: err.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    DivisionByZero,
    NonFiniteResult,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "除数不能为零"),
            Self::NonFiniteResult => write!(f, "计算结果不是有限数值"),
        }
    }
}
impl std::error::Error for EvalError {}
