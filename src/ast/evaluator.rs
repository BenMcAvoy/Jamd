use super::{
    BinaryExpression, BinaryOperatorKind, Expression, ExpressionKind, Statement, StatementKind,
    Visitor,
};

#[derive(Default)]
pub struct Evaluator {
    pub last_value: Option<i64>,
}

impl Visitor for Evaluator {
    fn visit_number(&mut self, number: i64) {
        self.last_value = Some(number);
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.expect("Don't care");
        self.visit_expression(&expr.right);
        let right = self.last_value.expect("Don't care");

        self.last_value = match expr.operator.kind {
            BinaryOperatorKind::Add => Some(left + right),
            BinaryOperatorKind::Subtract => Some(right - left),
            BinaryOperatorKind::Multiply => Some(left * right),
            BinaryOperatorKind::Divide => Some(left / right),
        };
    }

    fn visit_parenthesized_expression(&mut self, expr: &super::ParenthesizedExpression) {
        self.visit_expression(expr.expression.as_ref());
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match &statement.kind {
            StatementKind::Expression(expression) => self.visit_expression(expression),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Number(number) => self.visit_number(number.number),
            ExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ExpressionKind::Parenthesized(expr) => self.visit_parenthesized_expression(expr),
        }
    }
}
