use super::air;
use super::identifier::{Identify, Symbolise};
use super::noir::*;

pub struct Normaliser {
}

impl Normaliser {
    pub fn new() -> Normaliser {
        Normaliser {}
    }
}

impl air::visitor::Visitor for Normaliser {
    type Return = Node;

    fn visit_block_expr(&mut self, block_expr: air::BlockExpr) -> Self::Return {
        BlockExpr::new(
            block_expr.identify(),
            noir::into_exprs(air::visitor::walk_exprs(self, block_expr.body())),
            Expr::from(self.visit_expr(block_expr.ret())),
        ).into()
    }

    fn visit_call_expr(&mut self, call_expr: air::CallExpr) -> Self::Return {
        CallExpr::new(
            call_expr.identify(),
            Expr::from(self.visit_expr(call_expr.target())),
            noir::into_exprs(air::visitor::walk_exprs(self, call_expr.arguments())),
        ).into()
    }

    fn visit_def_expr(&mut self, for_expr: air::DefExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_for_expr(&mut self, for_expr: air::ForExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_function(&mut self, function: air::Function) -> Self::Return {
        unimplemented!()
    }

    fn visit_if_expr(&mut self, if_expr: air::IfExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_item_expr(&mut self, item_expr: air::ItemExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_lambda_type(&mut self, lambda_type: air::LambdaType) -> Self::Return {
        unimplemented!()
    }

    fn visit_literal_expr(&mut self, literal_expr: air::LiteralExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_module(&mut self, module: air::Module) -> Self::Return {
        unimplemented!()
    }

    fn visit_primitive_type(&mut self, primitive_type: air::PrimitiveType) -> Self::Return {
        unimplemented!()
    }

    fn visit_ref_expr(&mut self, ref_expr: air::RefExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_ref_type(&mut self, ref_type: air::RefType) -> Self::Return {
        unimplemented!()
    }

    fn visit_struct_expr(&mut self, struct_expr: air::StructExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_struct_element_expr(&mut self, struct_element_expr: air::StructElementExpr) -> Self::Return {
        unimplemented!()
    }

    fn visit_struct_type(&mut self, struct_type: air::StructType) -> Self::Return {
        unimplemented!()
    }

    fn visit_unresolved_type(&mut self, unresolved_type: air::UnresolvedType) -> Self::Return {
        unimplemented!()
    }

    fn visit_variable(&mut self, variable: air::Variable) -> Self::Return {
        unimplemented!()
    }

    fn visit_void_expr(&mut self, void_expr: air::VoidExpr) -> Self::Return {
        Node::Expr(Expr::Void(VoidExpr::new(void_expr.identify())))
    }
}

