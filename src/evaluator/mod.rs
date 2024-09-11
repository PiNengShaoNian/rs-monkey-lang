pub mod object;

use self::object::*;
use crate::ast::*;

#[derive(Debug)]
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&mut self, program: Program) -> Object {
        self.eval_block_stmt(program).unwrap_or(Object::Null)
    }

    fn eval_block_stmt(&mut self, stmts: BlockStmt) -> Option<Object> {
        stmts.into_iter().fold(None, |_, x| self.eval_stmt(x))
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            _ => None,
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Literal(literal) => self.eval_literal(literal),
            _ => None,
        }
    }

    fn eval_literal(&mut self, literal: Literal) -> Option<Object> {
        match literal {
            Literal::Int(value) => Some(Object::Int(value)),
            Literal::Bool(value) => Some(Object::Bool(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::evaluator::object::*;
    use crate::evaluator::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval(input: &str) -> Object {
        Evaluator::new().eval(Parser::new(Lexer::new(input)).parse())
    }

    #[test]
    fn test_integer_expr() {
        let tests = vec![("5", Object::Int(5)), ("10", Object::Int(10))];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_boolean_expr() {
        let tests = vec![("true", Object::Bool(true)), ("false", Object::Bool(false))];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }
}
