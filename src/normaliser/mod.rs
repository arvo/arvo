use super::air;
use super::identifier::Identify;
use super::noir::*;

pub struct Normaliser {
    context: Context,
}

impl Normaliser {
    pub fn new(context: Context) -> Normaliser {
        Normaliser {
            context: context
        }
    }

    pub fn normalise_function(&mut self, function: &air::Function) -> Function {
        Function::new(
            function.symbolise(),
            normalise_function_formals(function.formals()),
            normalise_function_ret(function.ret()),
            normalise_function_body(function.body()),
        )
    }

    pub fn normalise_function_formals(&mut self, formals: &air::Variables) -> Vec<Variable> {
        formals.iter().map(|formal| self.normalise_variable(formal)).collect()
    }

    pub fn normalise_function_ret(&mut self, ret: &air::Type) -> Type {
        self.normalise_type(ret)
    }

    pub fn normalise_function_body(&mut self, body: &air::Expr) -> BlockExpr {
        if let air::Expr::Block(ref block_expr) = *body {
            self.normalise_block_expr(block_expr)
        } else {
            BlockExpr::new(
                Identifier::id(),
                Exprs::new(),
                self.normalise_expr(body),
            )
        }
    }

    pub fn normalise_void_expr(&mut self, void_expr: &air::VoidExpr) -> VoidExpr {
        VoidExpr::new(void_expr.identify())
    }
}