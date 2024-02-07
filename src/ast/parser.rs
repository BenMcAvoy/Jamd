use crate::ast::lexer::{Lexer, Token};
use crate::ast;

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn from_input(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        Self { tokens, current: 0 }
    }

    pub fn next_statement(&mut self) -> Option<ast::Statement> {
        let token = self.current();

        let expr = self.parse_expression()

        todo!()
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }
}
