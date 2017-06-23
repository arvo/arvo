use super::*;

pub trait Visitor: Sized {
    type Return: Default;

    fn visit_block_expr(&mut self, block_expr: BlockExpr) -> Self::Return;
    fn visit_call_expr(&mut self, call_expr: CallExpr) -> Self::Return;
    fn visit_def_expr(&mut self, def_expr: DefExpr) -> Self::Return;
    fn visit_expr(&mut self, expr: Expr) -> Self::Return {
        walk_expr(self, expr)
    }
    fn visit_for_expr(&mut self, for_expr: ForExpr) -> Self::Return;
    fn visit_function(&mut self, function: Function) -> Self::Return;
    fn visit_if_expr(&mut self, if_expr: IfExpr) -> Self::Return;
    fn visit_item_expr(&mut self, item_expr: ItemExpr) -> Self::Return;
    fn visit_lambda_type(&mut self, lambda_type: LambdaType) -> Self::Return;
    fn visit_literal_expr(&mut self, literal_expr: LiteralExpr) -> Self::Return;
    fn visit_module(&mut self, module: Module) -> Self::Return;
    fn visit_primitive_type(&mut self, primitive_type: PrimitiveType) -> Self::Return;
    fn visit_ref_expr(&mut self, ref_expr: RefExpr) -> Self::Return;
    fn visit_ref_type(&mut self, ref_type: RefType) -> Self::Return;
    fn visit_struct_expr(&mut self, struct_expr: StructExpr) -> Self::Return;
    fn visit_struct_element_expr(&mut self,
                                 struct_element_expr: StructElementExpr)
                                 -> Self::Return;
    fn visit_struct_type(&mut self, struct_type: StructType) -> Self::Return;
    fn visit_type(&mut self, ty: Type) -> Self::Return {
        walk_type(self, ty)
    }
    fn visit_unresolved_type(&mut self, unresolved_type: UnresolvedType) -> Self::Return;
    fn visit_variable(&mut self, variable: Variable) -> Self::Return;
    fn visit_void_expr(&mut self, void_expr: VoidExpr) -> Self::Return;
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: Expr) -> V::Return {
    match expr {
        Expr::Block(block_expr) => visitor.visit_block_expr(block_expr),
        Expr::Call(call_expr) => visitor.visit_call_expr(call_expr),
        Expr::Def(def_expr) => visitor.visit_def_expr(def_expr),
        Expr::For(for_expr) => visitor.visit_for_expr(for_expr),
        Expr::If(if_expr) => visitor.visit_if_expr(if_expr),
        Expr::Item(item_expr) => visitor.visit_item_expr(item_expr),
        Expr::Literal(literal_expr) => visitor.visit_literal_expr(literal_expr),
        Expr::Ref(ref_expr) => visitor.visit_ref_expr(ref_expr),
        Expr::Struct(struct_expr) => visitor.visit_struct_expr(struct_expr),
        Expr::StructElement(struct_element_expr) => {
            visitor.visit_struct_element_expr(struct_element_expr)
        }
        Expr::Void(void_expr) => visitor.visit_void_expr(void_expr),
    }
}

pub fn walk_type<V: Visitor>(visitor: &mut V, ty: Type) -> V::Return {
    match ty {
        Type::Primitive(primitive_type) => visitor.visit_primitive_type(primitive_type),
        Type::Lambda(lambda_type) => visitor.visit_lambda_type(lambda_type),
        Type::Ref(ref_type) => visitor.visit_ref_type(ref_type),
        Type::Struct(struct_type) => visitor.visit_struct_type(struct_type),
        Type::Unresolved(unresolved_type) => visitor.visit_unresolved_type(unresolved_type),
    }
}