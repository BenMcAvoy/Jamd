use termion::color::{self, Fg, Reset};

use super::lexer::{TextSpan, Token};

#[derive(Default, Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn Visitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) {
        let mut printer = Printer::default();
        self.visit(&mut printer);
        println!("{}", printer.result);
    }
}

pub trait Visitor {
    fn visit_statement(&mut self, statement: &Statement) {
        match &statement.kind {
            StatementKind::Expression(expr) => self.visit_expression(expr),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Number(number) => self.visit_number(number),
            ExpressionKind::Binary(binary) => self.visit_binary_expression(binary),
            ExpressionKind::Parenthesized(parenthesized) => self.visit_parenthesized_expression(parenthesized),
            ExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_number(&mut self, number: &NumberExpression);
    fn visit_binary_expression(&mut self, expr: &BinaryExpression);
    fn visit_parenthesized_expression(&mut self, expr: &ParenthesizedExpression);
    fn visit_error(&mut self, expr: &TextSpan);
}

#[derive(Default)]
pub struct Printer {
    // indent: usize,
    result: String,
}

impl Printer {
    const NUMBER_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::White = color::White;

    fn add_whitespace(&mut self) {
        self.result.push(' ');
    }

    fn add_newline(&mut self) {
        self.result.push('\n');
    }

    // fn print_with_indent(&self, message: &str) {
    //     println!("{}{}", " ".repeat(self.indent), message);
    // }
}

impl Visitor for Printer {
    // fn visit_statement(&mut self, statement: &Statement) {
    //     self.print_with_indent("Statement:");
    //     self.indent += INDENT_SIZE;
    //     match &statement.kind {
    //         StatementKind::Expression(expression) => self.visit_expression(expression),
    //     }
    //     self.indent -= INDENT_SIZE;
    // }

    fn visit_statement(&mut self, statement: &Statement) {
        match &statement.kind {
            StatementKind::Expression(expr) => self.visit_expression(expr),
        }

        self.result.push_str(&format!("{}", Fg(Reset)));
    }

    fn visit_number(&mut self, number: &NumberExpression) {
        self.result.push_str(&format!(
                "{}{}",
                color::Fg(Self::NUMBER_COLOR),
                number.number,
                ));
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.result
            .push_str(&format!("{}{}", Fg(Self::TEXT_COLOR), span.literal));
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        self.visit_expression(&expr.left);
        self.add_whitespace();
        self.result.push_str(&format!(
                "{}{}",
                color::Fg(Self::TEXT_COLOR),
                expr.operator.token.span.literal
                ));
        self.add_whitespace();
        self.visit_expression(&expr.right);
    }

    fn visit_parenthesized_expression(&mut self, expr: &ParenthesizedExpression) {
        self.result
            .push_str(&format!("{}(", color::Fg(Self::TEXT_COLOR)));

        self.visit_expression(&expr.expression);

        self.result
            .push_str(&format!("{})", color::Fg(Self::TEXT_COLOR)));

        self.visit_expression(&expr.expression);
    }
}

const INDENT_SIZE: usize = 2;

impl Printer {}

// Statement
#[derive(Debug)]
pub enum StatementKind {
    Expression(Expression),
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
}

impl Statement {
    pub const fn new(kind: StatementKind) -> Self {
        Self { kind }
    }

    pub const fn expression(expression: Expression) -> Self {
        Self::new(StatementKind::Expression(expression))
    }
}

// Expression
#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    Number(NumberExpression),
    Binary(BinaryExpression),
    Parenthesized(ParenthesizedExpression),
    Error(TextSpan),
}

#[derive(Debug, PartialEq, Eq)]
pub struct NumberExpression {
    pub number: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParenthesizedExpression {
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinaryOperator,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub token: Token,
}

impl BinaryOperator {
    pub const fn precedence(&self) -> u8 {
        match self.kind {
            BinaryOperatorKind::Add | BinaryOperatorKind::Subtract | BinaryOperatorKind::Mod => 1,
            BinaryOperatorKind::Multiply | BinaryOperatorKind::Divide => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    pub kind: ExpressionKind,
}

impl Expression {
    pub const fn new(kind: ExpressionKind) -> Self {
        Self { kind }
    }

    pub const fn number(number: i64) -> Self {
        Self::new(ExpressionKind::Number(NumberExpression { number }))
    }

    pub const fn error(span: TextSpan) -> Self {
        Self::new(ExpressionKind::Error(span))
    }

    pub fn binary(left: Self, right: Self, operator: BinaryOperator) -> Self {
        Self::new(ExpressionKind::Binary(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        }))
    }

    pub fn parenthesized(kind: ExpressionKind) -> Self {
        Self::new(ExpressionKind::Parenthesized(ParenthesizedExpression {
            expression: Box::new(Self::new(kind)),
        }))
    }
}
