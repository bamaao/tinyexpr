use crate::error::LexError;
use crate::token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize, // byte offset; all recognized syntax is ASCII
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.bump_char();
                continue;
            }

            let start = self.pos;
            let kind = match ch {
                '+' => { self.bump_char(); TokenKind::Plus }
                '-' => { self.bump_char(); TokenKind::Minus }
                '*' => { self.bump_char(); TokenKind::Star }
                '/' => { self.bump_char(); TokenKind::Slash }
                '^' => { self.bump_char(); TokenKind::Caret }
                '(' => { self.bump_char(); TokenKind::LParen }
                ')' => { self.bump_char(); TokenKind::RParen }
                '0'..='9' | '.' => self.lex_number()?,
                _ => {
                    self.bump_char();
                    return Err(LexError {
                        message: format!("不支持的字符 `{ch}`"),
                        span: Span::new(start, self.pos),
                    });
                }
            };
            tokens.push(Token::new(kind, Span::new(start, self.pos)));
        }

        tokens.push(Token::new(TokenKind::Eof, Span::new(self.pos, self.pos)));
        Ok(tokens)
    }

    fn lex_number(&mut self) -> Result<TokenKind, LexError> {
        let start = self.pos;
        let mut seen_dot = false;
        let mut digits = 0usize;

        while let Some(ch) = self.peek_char() {
            match ch {
                '0'..='9' => {
                    digits += 1;
                    self.bump_char();
                }
                '.' if !seen_dot => {
                    seen_dot = true;
                    self.bump_char();
                }
                _ => break,
            }
        }

        if digits == 0 {
            return Err(LexError {
                message: "小数点必须至少有一侧包含数字".into(),
                span: Span::new(start, self.pos),
            });
        }

        let raw = &self.input[start..self.pos];
        let value = raw.parse::<f64>().map_err(|_| LexError {
            message: format!("无效数字 `{raw}`"),
            span: Span::new(start, self.pos),
        })?;

        if !value.is_finite() {
            return Err(LexError {
                message: "数字超出有限浮点数范围".into(),
                span: Span::new(start, self.pos),
            });
        }
        Ok(TokenKind::Number(value))
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }
}
