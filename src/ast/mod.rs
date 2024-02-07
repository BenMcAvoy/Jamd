pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<Statement>
}

// Statements
pub enum StatementKind {
    Expression(Expression)
}

pub struct Statement {
    kind: StatementKind
}

// Expressions
pub enum ExpressionKind {
    Number(i64)
}

pub struct Expression {
    kind: ExpressionKind
}

