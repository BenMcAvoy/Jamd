pub mod printer;

use crate::ast::lexer::{TextSpan, Token, TokenKind};
use std::{cell::RefCell, rc::Rc};

pub type BagCell = Rc<RefCell<Bag>>;

pub enum DiagnosticKind {
    Warning,
    Error,
}

pub struct Diagnostic {
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub const fn new(message: String, span: TextSpan, kind: DiagnosticKind) -> Self {
        Self {
            message,
            span,
            kind,
        }
    }
}

#[derive(Default)]
pub struct Bag {
    pub(crate) diagnostics: Vec<Diagnostic>,
}

impl Bag {
    pub fn report_error(&mut self, message: String, span: TextSpan) {
        self.diagnostics
            .push(Diagnostic::new(message, span, DiagnosticKind::Error));
    }

    pub fn report_warning(&mut self, message: String, span: TextSpan) {
        self.diagnostics
            .push(Diagnostic::new(message, span, DiagnosticKind::Warning));
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenKind, actual: &Token) {
        self.report_error(
            format!("Expected <{expected}>, found <{}>", actual.kind),
            actual.span.clone(),
        );
    }

    pub fn report_expected_expression(&mut self, token: &Token) {
        self.report_error(
            format!("Expected expression, found <{}>", token.kind),
            token.span.clone(),
        );
    }
}
