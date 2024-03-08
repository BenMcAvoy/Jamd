// #![cfg_attr(debug_assertions, allow(dead_code))]

use crate::ast::lexer::{Lexer, Token, TokenKind};
use crate::ast::{Expression, Statement};
use crate::diagnostics::BagCell;

use super::counter::Counter;
use super::{BinaryOperator, BinaryOperatorKind};

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: Counter,
    bag: BagCell,
}

// TODO: Refactor into implementing iterators
impl Iterator for Parser {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}

impl Parser {
    pub fn from_input(input: &str, bag: BagCell) -> Self {
        let tokens: Vec<_> = Lexer::new(input)
            .filter(|t| t.kind != TokenKind::Whitespace)
            .collect();

        Self {
            tokens,
            current: Counter::default(),
            bag,
        }
    }

    pub fn next_statement(&mut self) -> Option<Statement> {
        if self.current().kind == TokenKind::Eof {
            return None;
        }

        Some(self.parse_statement())
    }

    fn parse_statement(&mut self) -> Statement {
        let expr = self.parse_expression();
        Statement::expression(expr)
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Expression {
        let mut left = self.parse_primary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            // Consume the operator
            self.consume();

            // Get the precedence
            let op_precedence = operator.precedence();

            // Break if the operator has lower precedence
            if op_precedence <= precedence {
                println!("Precedence is lower than current, breaking");
                break;
            }

            // Parse the right hand side
            let right = self.parse_binary_expression(op_precedence);

            // Combine the left and right hand side
            left = Expression::binary(left, right, operator);
        }

        left
    }

    fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.current();

        let kind = match token.kind {
            TokenKind::Plus => Some(BinaryOperatorKind::Add),
            TokenKind::Minus => Some(BinaryOperatorKind::Subtract),
            TokenKind::Asterisk => Some(BinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(BinaryOperatorKind::Divide),
            TokenKind::Mod => Some(BinaryOperatorKind::Mod),
            _ => None,
        };

        // Do this without the `new` function
        kind.map(|kind| BinaryOperator {
            token: token.clone(),
            kind,
        })
    }

    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.consume();

        match token.kind {
            TokenKind::Number(n) => Expression::number(n),
            TokenKind::LeftParen => {
                let expr = self.parse_expression();
                self.consume_and_check(&TokenKind::RightParen);

                Expression::parenthesized(expr.kind)
            }

            _ => {
                self.bag.borrow_mut().report_expected_expression(token);
                Expression::error(token.span.clone())
            }
        }
    }

    fn peek(&self, offset: isize) -> &Token {
        // let index = self.current.get() + offset as usize;
        let index = self.current.wrapping_add_signed(offset) % self.tokens.len();
        self.tokens.get(index).expect("Out of bounds")
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn consume(&self) -> &Token {
        self.current.increment();
        self.peek(-1)
    }

    fn consume_and_check(&self, kind: &TokenKind) -> &Token {
        let token = self.consume();

        if token.kind != *kind {
            // eprintln!("Unexpected token: {token:?}");
            self.bag.borrow_mut().report_unexpected_token(kind, token);
        }

        token
    }
}
