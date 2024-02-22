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
    }
}

pub trait Visitor {
    fn visit_statement(&mut self, statement: &Statement);
    fn visit_expression(&mut self, expression: &Expression);
    fn visit_number(&mut self, number: i64);
    fn visit_binary_expression(&mut self, expr: &BinaryExpression);
    fn visit_parenthesized_expression(&mut self, expr: &ParenthesizedExpression);
    fn visit_error_expression(&mut self, expr: &TextSpan);
}

#[derive(Default)]
pub struct Printer {
    indent: usize,
}

impl Visitor for Printer {
    fn visit_number(&mut self, number: i64) {
        self.print_with_indent(&format!("Number: {number}"));
    }

    fn visit_statement(&mut self, statement: &Statement) {
        self.print_with_indent("Statement:");
        self.indent += INDENT_SIZE;
        match &statement.kind {
            StatementKind::Expression(expression) => self.visit_expression(expression),
        }
        self.indent -= INDENT_SIZE;
    }

    fn visit_expression(&mut self, expression: &Expression) {
        self.print_with_indent("Expression:");
        self.indent += INDENT_SIZE;

        match &expression.kind {
            ExpressionKind::Number(number) => self.visit_number(number.number),
            ExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ExpressionKind::Parenthesized(expr) => self.visit_parenthesized_expression(expr),
            ExpressionKind::Error(expr) => self.visit_error_expression(expr),
        }

        self.indent -= INDENT_SIZE;
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        self.print_with_indent("Binary expression:");
        self.indent += INDENT_SIZE;
        self.print_with_indent(&format!("Operator: {:?}", expr.operator.kind));
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);
        self.indent -= INDENT_SIZE;
    }

    fn visit_parenthesized_expression(&mut self, expr: &ParenthesizedExpression) {
        self.print_with_indent("Parenthesized expression:");
        self.indent += INDENT_SIZE;
        self.visit_expression(&expr.expression);
        self.indent -= INDENT_SIZE;
    }

    fn visit_error_expression(&mut self, expr: &TextSpan) {
        println!("Error expression: {}", expr.literal);
    }
}

const INDENT_SIZE: usize = 2;

impl Printer {
    fn print_with_indent(&self, message: &str) {
        println!("{}{}", " ".repeat(self.indent), message);
    }
}

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
