use super::air;
use super::air::Typedef;
use super::identifier::{Identifier, Identify, Symbolise};
use super::noir::*;
use super::noir::context::*;

pub struct Normaliser {
    context: Context,
}

impl Normaliser {
    pub fn new(context: Context) -> Normaliser {
        Normaliser {
            context: context
        }
    }

    pub fn normalise_block_expr(&mut self, block_expr: &air::BlockExpr) -> BlockExpr {
        BlockExpr::new(
            block_expr.identify(),
            self.normalise_exprs(block_expr.body()),
            self.normalise_expr(block_expr.ret()),
            FunctionTable::new(),
            TypeTable::new(),
            VariableTable::new(),
        )
    }

    pub fn normalise_function(&mut self, function: &air::Function) -> Function {
        let body = function.body().and_then(|body| if let air::Expr::Block(block_expr) = body {
            Some(*block_expr.clone())
        } else {
            Some(air::BlockExpr::new(
                Identifier::id(),
                air::Exprs::new(),
                body.clone(),
            ))
        });
        Function::new(
            function.symbolise(),
            self.normalise_variables(function.formals()),
            self.normalise_type(function.ret()),
            body.and_then(|body| Some(self.normalise_block_expr(&body))),
        )
    }

    pub fn normalise_lambda_type(&mut self, lambda_type: &air::LambdaType) -> LambdaType {
        LambdaType::new(
            self.normalise_types(lambda_type.formals()),
            self.normalise_type(lambda_type.ret()),
        )
    }

    pub fn normalise_variables(&mut self, variables: &air::Variables) -> Variables {
        variables.iter().map(|variable| self.normalise_variable(variable)).collect()
    }

    pub fn normalise_variable(&mut self, variable: &air::Variable) -> Variable {
        Variable::new(
            variable.symbolise(),
            self.normalise_type(&variable.typedef()),
        )
    }

    pub fn normalise_void_expr(&mut self, void_expr: &air::VoidExpr) -> VoidExpr {
        VoidExpr::new(void_expr.identify())
    }

    pub fn normalise_expr(&mut self, expr: &air::Expr) -> Expr {
        match *expr {
            air::Expr::Block(ref block_expr) => self.normalise_block_expr(block_expr),
            _ => unimplemented!(),
        }.into()
    }

    pub fn normalise_exprs(&mut self, exprs: &air::Exprs) -> Exprs {
        exprs.iter().map(|expr| self.normalise_expr(expr)).collect()
    }

    pub fn normalise_type(&mut self, expr: &air::Type) -> Type {
        match *expr {
            air::Type::Lambda(ref lambda_type) => self.normalise_lambda_type(lambda_type),
            _ => unimplemented!(),
        }.into()
    }

    pub fn normalise_types(&mut self, types: &air::Types) -> Types {
        types.iter().map(|ty| self.normalise_type(ty)).collect()
    }
}