use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::error::ParseError;
use crate::token::{Span, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_additive()?;
        if !matches!(self.peek().kind, TokenKind::Eof) {
            return Err(self.error_here("表达式结束后存在多余内容"));
        }
        Ok(expr)
    }

    // Additive ::= Multiplicative { ("+" | "-") Multiplicative }
    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_multiplicative()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_multiplicative()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    // Multiplicative ::= Unary { ("*" | "/") Unary }
    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_unary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    // Unary ::= ("+" | "-") Unary | Power
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind {
            TokenKind::Plus => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Plus,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            TokenKind::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_power(),
        }
    }

    // Power ::= Primary [ "^" Unary ]
    // Parsing the RHS as Unary makes exponentiation right-associative and
    // permits negative exponents, while keeping -2^2 == -(2^2).
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_primary()?;
        if matches!(self.peek().kind, TokenKind::Caret) {
            self.advance();
            let right = self.parse_unary()?;
            Ok(Expr::Binary {
                op: BinaryOp::Power,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    // Primary ::= Number | "(" Expression ")"
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind.clone() {
            TokenKind::Number(value) => {
                self.advance();
                Ok(Expr::Number(value))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_additive()?;
                if !matches!(self.peek().kind, TokenKind::RParen) {
                    return Err(self.error_here("缺少右括号 `)`"));
                }
                self.advance();
                Ok(expr)
            }
            _ => Err(self.error_here("此处需要数字或左括号")),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        let index = self.current;
        if !matches!(self.tokens[index].kind, TokenKind::Eof) {
            self.current += 1;
        }
        &self.tokens[index]
    }

    fn error_here(&self, message: &str) -> ParseError {
        let span = self.peek().span;
        ParseError {
            message: message.into(),
            span: Span::new(span.start, span.end),
        }
    }
}
