#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::ast::lexer::{Lexer, Token, TokenKind};
use crate::ast::{Expression, Statement};

use super::{BinaryOperator, BinaryOperatorKind};

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// TODO: Refactor into implementing iterators
impl Iterator for Parser {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}

impl Parser {
    pub fn from_input(input: &str) -> Self {
        let tokens: Vec<_> = Lexer::new(input)
            .filter(|t| t.kind != TokenKind::Whitespace)
            .collect();

        Self { tokens, current: 0 }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn next_statement(&mut self) -> Option<Statement> {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.current()?.kind == TokenKind::Eof {
            return None;
        }

        // let _token = self.current()?;
        let expr = self.parse_expression()?;

        Some(Statement::expression(expr))
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<Expression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_binary_operator() {
            self.consume()?;
            let op_precedence = operator.precedence();

            if op_precedence <= precedence {
                println!("Precedence is lower than current, breaking");
                break;
            }

            let right = self.parse_binary_expression(op_precedence)?;

            left = Expression::binary(right, left, operator);
        }

        Some(left)
    }

    fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.current()?;

        let kind = match token.kind {
            TokenKind::Plus => Some(BinaryOperatorKind::Add),
            TokenKind::Minus => Some(BinaryOperatorKind::Subtract),
            TokenKind::Asterisk => Some(BinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(BinaryOperatorKind::Divide),
            _ => None,
        };

        // Do this without the `new` function
        kind.map(|kind| BinaryOperator {
            token: token.clone(),
            kind,
        })
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        let token = self.consume()?;

        match token.kind {
            TokenKind::Number(n) => Some(Expression::number(n)),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;

                assert_eq!(self.consume()?.kind, TokenKind::RightParen);

                // Some(expr)
                Some(Expression::parenthesized(expr.kind))
            }

            _ => {
                println!("Unexpected token: {token:?}");
                None
            }
        }
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        let index = self.current.wrapping_add_signed(offset);
        self.tokens.get(index)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        Some(token)
    }
}
