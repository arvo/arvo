use super::air;
use super::identifier::{Identifier, Identify, Symbolise};
use super::noir::*;

pub struct Normaliser {
}

impl Normaliser {
    pub fn new(context: Context) -> Normaliser {
        Normaliser {
            context: context
        }
    }

    pub fn normalise_function(&mut self, function: air::Function) -> Function {
        Function::new(
            function.symbolise(),
            self.normalise_function_formals(function.formals()),
            self.normalise_function_ret(function.ret()),
            self.normalise_function_body(function.body()),
        )
    }

    pub fn normalise_function_formals(&mut self, formals: air::Variables) -> Vec<Variable> {
        formals.iter().map(|formal| self.normalise_variable(formal)).collect()
    }

    pub fn normalise_function_ret(&mut self, ret: air::Type) -> Type {
        self.normalise_type(ret)
    }

    pub fn normalise_function_body(&mut self, body: air::Expr) -> BlockExpr {
        if let air::Expr::Block(block_expr) = body {
            self.normalise_block_expr(block_expr)
        } else {
            BlockExpr::new(
                Identifier::id(),
                Exprs::new(),
                self.normalise_expr(body),
            )
        }
    }

    pub fn normalise_block_expr(&mut self, block_expr: air::BlockExpr) -> BlockExpr {
        BlockExpr::new(
            block_expr.identify(),
            self.normalise_exprs(block_expr.body()),
            self.normalise_expr(block_expr.ret()),
        )
    }

    pub fn normalise_void_expr(&mut self, void_expr: air::VoidExpr) -> VoidExpr {
        VoidExpr::new(void_expr.identify())
    }

    pub fn normalise_expr(&mut self, expr: air::Expr) -> Expr {
        match expr {
            air::Expr::Block(block_expr) => Expr::Block(self.normalise_block_expr(block_expr)),
            air::Expr::Void(void_expr) => Expr::Void(self.normalise_void_expr(void_expr)),
            _ => unimplemented!(),
        }
    }

    pub fn normalise_exprs(&mut self, exprs: air::Exprs) -> Exprs {
        exprs.into_iter().map(|expr| self.normalise_expr(expr)).collect()
    }

    pub fn normalise_type(&mut self, ty: air::Type) -> Expr {
        match type {
            air::Type::Primitive(primitive_type) => self.normalise_primitive_type(primitive_type),
            _ => unimplemented!(),
        }
    }

    pub fn normalise_types(&mut self, types: air::Types) -> Types {
        types.into_iter().map(|ty| self.normalise_type(ty)).collect()
    }
}