//! Resolver

use super::air;
use super::ast;

pub struct Resolver {
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
        }
    }

    pub fn resolve(&mut self, node: ast::Ast) -> air::Air {
        match node {
            ast::Ast::Expr(expr) => self.resolve_expr(expr).into(),
            _ => unimplemented!(),
        }
    }

    pub fn resolve_expr(&mut self, expr: ast::Expr) -> air::Expr {
        match expr {
            ast::Expr::Operator(op_expr) => self.resolve_operator_expr(*op_expr).into(),
            _ => unimplemented!(),
        }
    }

    pub fn resolve_operator_expr(&mut self, op_expr: ast::OperatorExpr) -> air::CallExpr {
        match op_expr {
            ast::OperatorExpr::Binary(bin_expr) => self.resolve_binary_expr(*bin_expr),
            _ => unimplemented!(),
        }
    }

    pub fn resolve_binary_expr(&mut self, bin_expr: ast::BinaryOperatorExpr) -> air::CallExpr {
        unimplemented!()
    }
}