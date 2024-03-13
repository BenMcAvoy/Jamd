use super::{
    lexer::TextSpan, BinaryExpression, BinaryOperatorKind, Expression, ExpressionKind,
    NumberExpression, Statement, StatementKind, Visitor,
};

#[derive(Default)]
pub struct Evaluator {
    pub last_value: Option<i64>,
    pub values: Vec<i64>,
}

impl Visitor for Evaluator {
    fn visit_number(&mut self, number: &NumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.expect("Don't care");
        self.visit_expression(&expr.right);
        let right = self.last_value.expect("Don't care");

        self.last_value = match expr.operator.kind {
            BinaryOperatorKind::Add => Some(left + right),
            BinaryOperatorKind::Subtract => Some(left - right),
            BinaryOperatorKind::Multiply => Some(left * right),
            BinaryOperatorKind::Divide => Some(left / right),
            BinaryOperatorKind::Mod => Some(left % right),
        };
    }

    fn visit_parenthesized_expression(&mut self, expr: &super::ParenthesizedExpression) {
        self.visit_expression(expr.expression.as_ref());
    }

    // fn visit_statement(&mut self, statement: &Statement) {
    //     match &statement.kind {
    //         StatementKind::Expression(expression) => self.visit_expression(expression),
    //     }

    //     if let Some(result) = self.last_value {
    //         self.values.push(result);
    //     }
    // }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Number(number) => self.visit_number(number),
            ExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ExpressionKind::Parenthesized(expr) => self.visit_parenthesized_expression(expr),
            ExpressionKind::Error(span) => self.visit_error(span),
        }
    }

    fn visit_error(&mut self, _span: &TextSpan) {
        println!("Cannot evaluate error expression");
    }
}
