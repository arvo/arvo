//! Abstract Syntax Tree
//!
//! The abstract intermediate representation (AIR) is resolved from an AST.
//! Resolution checks that symbols are brought into scope before they are
//! used, and are not used after they exit scope. At this stage, types have
//! not necessarily been resolved.

pub use super::bair::PrimitiveType;

use super::bair::Object;
use super::identifier::{Identifier, Identify, Symbol, Symbolise};

#[macro_use]
pub mod macros;
pub mod prelude;

/// The `Typedef` trait is implemented by nodes that can express some `Type`.
pub trait Typedef {
    fn typedef(&self) -> Type;
}

/// A `BlockExpr` contains a set of `Exprs` that will be evaluated in a
/// non-deterministic order. It also contains a return `Expr`.
#[derive(Clone)]
pub struct BlockExpr(Object<_BlockExpr>);

impl BlockExpr {
    pub fn new(identifier: Identifier, body: Exprs, ret: Expr) -> BlockExpr {
        BlockExpr(object!(_BlockExpr::new(identifier, body, ret)))
    }

    pub fn body(&self) -> Exprs {
        object_proxy![self.0 => body().clone()]
    }

    pub fn ret(&self) -> Expr {
        object_proxy![self.0 => ret().clone()]
    }
}

impl Identify for BlockExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for BlockExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => ret().typedef()]
    }
}

#[derive(Clone)]
struct _BlockExpr {
    identifier: Identifier,
    body: Exprs,
    ret: Expr,
}

impl _BlockExpr {
    fn new(identifier: Identifier, body: Exprs, ret: Expr) -> _BlockExpr {
        _BlockExpr {
            identifier: identifier,
            body: body,
            ret: ret,
        }
    }

    fn body(&self) -> &Exprs {
        &self.body
    }

    fn ret(&self) -> &Expr {
        &self.ret
    }
}

impl Identify for _BlockExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _BlockExpr {
    fn typedef(&self) -> Type {
        self.ret.typedef()
    }
}

/// A `CallExpr` is a function call to some target function. The target
/// function is an `Expr` to allow for higher-order functions to be used as
/// call targets. It contains a set of argument `Exprs` that will be passed to
/// the function call.
#[derive(Clone)]
pub struct CallExpr(Object<_CallExpr>);

impl CallExpr {
    pub fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> CallExpr {
        CallExpr(object!(_CallExpr::new(identifier, target, arguments)))
    }

    pub fn target(&self) -> Expr {
        object_proxy![self.0 => target().clone()]
    }

    pub fn arguments(&self) -> Exprs {
        object_proxy![self.0 => arguments().clone()]
    }
}

impl Identify for CallExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for CallExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _CallExpr {
    identifier: Identifier,
    target: Expr,
    arguments: Exprs,
}

impl _CallExpr {
    fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> _CallExpr {
        _CallExpr {
            identifier: identifier,
            target: target,
            arguments: arguments,
        }
    }

    fn target(&self) -> &Expr {
        &self.target
    }

    fn arguments(&self) -> &Exprs {
        &self.arguments
    }
}

impl Identify for _CallExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _CallExpr {
    fn typedef(&self) -> Type {
        if let Type::Lambda(lambda_ty) = self.target.typedef() {
            return lambda_ty.ret();
        }
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct DefExpr(Object<_DefExpr>);

impl DefExpr {
    pub fn new(identifier: Identifier, variable: Variable, definition: Expr) -> DefExpr {
        DefExpr(object!(_DefExpr::new(identifier, variable, definition)))
    }
}

impl Identify for DefExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for DefExpr {
    fn typedef(&self) -> Type {
        Type::Primitive(PrimitiveType::Void)
    }
}

#[derive(Clone)]
struct _DefExpr {
    identifier: Identifier,
    variable: Variable,
    definition: Expr,
}

impl _DefExpr {
    fn new(identifier: Identifier, variable: Variable, definition: Expr) -> _DefExpr {
        _DefExpr {
            identifier: identifier,
            variable: variable,
            definition: definition,
        }
    }
}

impl Identify for _DefExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub enum Expr {
    Block(BlockExpr),
    Call(CallExpr),
    Def(DefExpr),
    For(ForExpr),
    If(IfExpr),
    Item(ItemExpr),
    Literal(LiteralExpr),
    Ref(RefExpr),
    Struct(StructExpr),
    StructElement(StructElementExpr),
    Void(VoidExpr),
}

impl Identify for Expr {
    fn identify(&self) -> Identifier {
        expr_proxy![*self => identify()]
    }
}

impl Typedef for Expr {
    fn typedef(&self) -> Type {
        expr_proxy![*self => typedef()]
    }
}

pub type Exprs = Vec<Expr>;

///
#[derive(Clone)]
pub struct ForExpr(Object<_ForExpr>);

impl ForExpr {
    pub fn new(identifier: Identifier,
               variable: Variable,
               iterand: Expr,
               iteration: BlockExpr)
               -> ForExpr {
        ForExpr(object!(_ForExpr::new(identifier, variable, iterand, iteration)))
    }
}

impl Identify for ForExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for ForExpr {
    fn typedef(&self) -> Type {
        unimplemented!()
    }
}

#[derive(Clone)]
struct _ForExpr {
    identifier: Identifier,
    variable: Variable,
    iterand: Expr,
    iteration: BlockExpr,
}

impl _ForExpr {
    fn new(identifier: Identifier,
           variable: Variable,
           iterand: Expr,
           iteration: BlockExpr)
           -> _ForExpr {
        _ForExpr {
            identifier: identifier,
            variable: variable,
            iterand: iterand,
            iteration: iteration,
        }
    }
}

impl Identify for _ForExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct Function(Object<_Function>);

pub type Functions = Vec<Function>;

impl Function {
    pub fn new(symbol: Symbol, formals: Variables, ret: Type, body: Option<Expr>) -> Function {
        Function(object!(_Function::new(symbol, formals, ret, body)))
    }
}

impl Identify for Function {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for Function {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

impl Typedef for Function {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _Function {
    symbol: Symbol,
    formals: Variables,
    ret: Type,
    body: Option<Expr>,
}

impl _Function {
    fn new(symbol: Symbol, formals: Variables, ret: Type, body: Option<Expr>) -> _Function {
        _Function {
            symbol: symbol,
            formals: formals,
            ret: ret,
            body: body,
        }
    }
}

impl Identify for _Function {
    fn identify(&self) -> Identifier {
        self.symbolise().identify()
    }
}

impl Symbolise for _Function {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for _Function {
    fn typedef(&self) -> Type {
        Type::Lambda(LambdaType::new(self.formals
                                         .iter()
                                         .map(|formal| formal.typedef())
                                         .collect(),
                                     self.ret.clone()))
    }
}

///
#[derive(Clone)]
pub struct IfExpr(Object<_IfExpr>);

impl IfExpr {
    pub fn new(identifier: Identifier,
               conditional_expr: Expr,
               then_block: BlockExpr,
               else_block: BlockExpr)
               -> IfExpr {
        IfExpr(object!(_IfExpr::new(identifier, conditional_expr, then_block, else_block)))
    }
}

impl Identify for IfExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for IfExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => then_block().typedef()]
    }
}

#[derive(Clone)]
struct _IfExpr {
    identifier: Identifier,
    conditional_expr: Expr,
    then_block: BlockExpr,
    else_block: BlockExpr,
}

impl _IfExpr {
    fn new(identifier: Identifier,
           conditional_expr: Expr,
           then_block: BlockExpr,
           else_block: BlockExpr)
           -> _IfExpr {
        _IfExpr {
            identifier: identifier,
            conditional_expr: conditional_expr,
            then_block: then_block,
            else_block: else_block,
        }
    }

    fn then_block(&self) -> &BlockExpr {
        &self.then_block
    }
}

impl Identify for _IfExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct ItemExpr(Object<_ItemExpr>);

impl ItemExpr {
    pub fn new(identifier: Identifier, item: Item) -> ItemExpr {
        ItemExpr(object!(_ItemExpr::new(identifier, item)))
    }
}

impl Identify for ItemExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for ItemExpr {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

impl Typedef for ItemExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _ItemExpr {
    identifier: Identifier,
    item: Item,
}

impl _ItemExpr {
    fn new(identifier: Identifier, item: Item) -> _ItemExpr {
        _ItemExpr {
            identifier: identifier,
            item: item,
        }
    }
}

impl Identify for _ItemExpr {
    fn identify(&self) -> Identifier {
        item_proxy![*&self.item => identify()]
    }
}

impl Symbolise for _ItemExpr {
    fn symbolise(&self) -> Symbol {
        item_proxy![*&self.item => symbolise()]
    }
}

impl Typedef for _ItemExpr {
    fn typedef(&self) -> Type {
        item_proxy![*&self.item => typedef()]
    }
}

#[derive(Clone)]
pub enum Item {
    Function(Function),
    Variable(Variable),
}

///
#[derive(Clone)]
pub struct LambdaType(Object<_LambdaType>);

impl LambdaType {
    pub fn new(formals: Types, ret: Type) -> LambdaType {
        LambdaType(object!(_LambdaType::new(formals, ret)))
    }

    pub fn ret(&self) -> Type {
        object_proxy![self.0 => ret().clone()]
    }
}

#[derive(Clone)]
struct _LambdaType {
    formals: Types,
    ret: Type,
}

impl _LambdaType {
    fn new(formals: Types, ret: Type) -> _LambdaType {
        _LambdaType {
            formals: formals,
            ret: ret,
        }
    }

    fn ret(&self) -> &Type {
        &self.ret
    }
}

///
#[derive(Clone)]
pub struct LiteralExpr(Object<_LiteralExpr>);

impl Identify for LiteralExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for LiteralExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _LiteralExpr {
    identifier: Identifier,
    literal: Literal,
}

impl Identify for _LiteralExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _LiteralExpr {
    fn typedef(&self) -> Type {
        self.literal.typedef()
    }
}

#[derive(Clone)]
pub enum Literal {
    Bool(bool),
    Char(char),
    F32(f32),
    F64(i64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Str(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
}

impl Typedef for Literal {
    fn typedef(&self) -> Type {
        match *self {
            Literal::Bool(..) => Type::Primitive(PrimitiveType::Bool),
            Literal::Char(..) => Type::Primitive(PrimitiveType::Char),
            Literal::F32(..) => Type::Primitive(PrimitiveType::F32),
            Literal::F64(..) => Type::Primitive(PrimitiveType::F64),
            Literal::I8(..) => Type::Primitive(PrimitiveType::I8),
            Literal::I16(..) => Type::Primitive(PrimitiveType::I16),
            Literal::I32(..) => Type::Primitive(PrimitiveType::I32),
            Literal::I64(..) => Type::Primitive(PrimitiveType::I64),
            Literal::Str(..) => Type::Primitive(PrimitiveType::Str),
            Literal::U8(..) => Type::Primitive(PrimitiveType::U8),
            Literal::U16(..) => Type::Primitive(PrimitiveType::U16),
            Literal::U32(..) => Type::Primitive(PrimitiveType::U32),
            Literal::U64(..) => Type::Primitive(PrimitiveType::U64),
            Literal::USize(..) => Type::Primitive(PrimitiveType::USize),
        }
    }
}

///
#[derive(Clone)]
pub struct Module(Object<_Module>);

pub type Modules = Vec<Module>;

impl Module {
    pub fn new(symbol: Symbol, functions: Functions, modules: Modules, types: Types) -> Module {
        Module(object!(_Module::new(symbol, functions, modules, types)))
    }
}

impl Identify for Module {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for Module {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

#[derive(Clone)]
struct _Module {
    symbol: Symbol,
    functions: Functions,
    modules: Modules,
    types: Types,
}

impl _Module {
    fn new(symbol: Symbol, functions: Functions, modules: Modules, types: Types) -> _Module {
        _Module {
            symbol: symbol,
            functions: functions,
            modules: modules,
            types: types,
        }
    }
}

impl Identify for _Module {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for _Module {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct RefExpr(Object<_RefExpr>);

impl RefExpr {
    pub fn new(identifier: Identifier, referent: Expr) -> RefExpr {
        RefExpr(object!(_RefExpr::new(identifier, referent)))
    }
}

impl Identify for RefExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for RefExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _RefExpr {
    identifier: Identifier,
    referent: Expr,
}

impl _RefExpr {
    fn new(identifier: Identifier, referent: Expr) -> _RefExpr {
        _RefExpr {
            identifier: identifier,
            referent: referent,
        }
    }
}

impl Identify for _RefExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _RefExpr {
    fn typedef(&self) -> Type {
        Type::Ref(RefType::new(self.referent.typedef()))
    }
}

///
#[derive(Clone)]
pub struct RefType(Object<_RefType>);

impl RefType {
    pub fn new(referent: Type) -> RefType {
        RefType(object!(_RefType::new(referent)))
    }
}

#[derive(Clone)]
struct _RefType {
    referent: Type,
}

impl _RefType {
    fn new(referent: Type) -> _RefType {
        _RefType { referent: referent }
    }
}

///
#[derive(Clone)]
pub struct StructExpr(Object<_StructExpr>);

impl StructExpr {
    pub fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> StructExpr {
        StructExpr(object!(_StructExpr::new(identifier, elements, ty)))
    }
}

impl Identify for StructExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for StructExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _StructExpr {
    identifier: Identifier,
    elements: Exprs,
    ty: StructType,
}

impl _StructExpr {
    fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> _StructExpr {
        _StructExpr {
            identifier: identifier,
            elements: elements,
            ty: ty,
        }
    }
}

impl Identify for _StructExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _StructExpr {
    fn typedef(&self) -> Type {
        Type::Struct(self.ty.clone())
    }
}

///
#[derive(Clone)]
pub struct StructElementExpr(Object<_StructElementExpr>);

impl StructElementExpr {
    pub fn new(identifier: Identifier, target: Expr, variable: Variable) -> StructElementExpr {
        StructElementExpr(object!(_StructElementExpr::new(identifier, target, variable)))
    }
}

impl Identify for StructElementExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for StructElementExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _StructElementExpr {
    identifier: Identifier,
    target: Expr,
    variable: Variable,
}

impl _StructElementExpr {
    fn new(identifier: Identifier, target: Expr, variable: Variable) -> _StructElementExpr {
        _StructElementExpr {
            identifier: identifier,
            target: target,
            variable: variable,
        }
    }
}

impl Identify for _StructElementExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _StructElementExpr {
    fn typedef(&self) -> Type {
        self.variable.typedef()
    }
}

///
#[derive(Clone)]
pub struct StructType(Object<_StructType>);

impl StructType {
    pub fn new(elements: Variables) -> StructType {
        StructType(object!(_StructType::new(elements)))
    }

    pub fn for_elements<T, F: FnMut(&mut Variable) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => elements_mut().iter_mut() loop element {
            f(element);
        }]
    }

    pub fn map_elements<T, F: FnMut(&mut Variable) -> T>(&self, f: F) -> Vec<T> {
        object_proxy_mut![self.0 => elements_mut().iter_mut().map(f).collect()]
    }

    pub fn num_elements(&self) -> usize {
        object_proxy![self.0 => elements().len()]
    }
}

#[derive(Clone)]
struct _StructType {
    elements: Variables,
}

impl _StructType {
    fn new(elements: Variables) -> _StructType {
        _StructType { elements: elements }
    }

    fn elements(&self) -> &Variables {
        &self.elements
    }

    fn elements_mut(&mut self) -> &mut Variables {
        &mut self.elements
    }
}

///
#[derive(Clone)]
pub enum Type {
    Lambda(LambdaType),
    Primitive(PrimitiveType),
    Ref(RefType),
    Struct(StructType),
    Unresolved(UnresolvedType),
}

pub type Types = Vec<Type>;

///
#[derive(Clone)]
pub struct Unresolved(Object<_Unresolved>);

impl Identify for Unresolved {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for Unresolved {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

impl Typedef for Unresolved {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

struct _Unresolved {
    symbol: Symbol,
}

impl Identify for _Unresolved {
    fn identify(&self) -> Identifier {
        self.symbolise().identify()
    }
}

impl Symbolise for _Unresolved {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for _Unresolved {
    fn typedef(&self) -> Type {
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct UnresolvedType(Object<_UnresolvedType>);

impl Symbolise for UnresolvedType {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

impl Typedef for UnresolvedType {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

struct _UnresolvedType {
    symbol: Symbol,
}

impl Symbolise for _UnresolvedType {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for _UnresolvedType {
    fn typedef(&self) -> Type {
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct Variable(Object<_Variable>);

impl Variable {
    pub fn new(symbol: Symbol, ty: Type) -> Variable {
        Variable(object!(_Variable::new(symbol, ty)))
    }
}

impl Identify for Variable {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for Variable {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

impl Typedef for Variable {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

pub type Variables = Vec<Variable>;

#[derive(Clone)]
struct _Variable {
    symbol: Symbol,
    ty: Type,
}

impl _Variable {
    fn new(symbol: Symbol, ty: Type) -> _Variable {
        _Variable {
            symbol: symbol,
            ty: ty,
        }
    }
}

impl Identify for _Variable {
    fn identify(&self) -> Identifier {
        self.symbolise().identify()
    }
}

impl Symbolise for _Variable {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for _Variable {
    fn typedef(&self) -> Type {
        self.ty.clone()
    }
}

///
#[derive(Clone)]
pub struct VoidExpr(Object<_VoidExpr>);

impl VoidExpr {
    pub fn new(identifier: Identifier) -> VoidExpr {
        VoidExpr(object!(_VoidExpr::new(identifier)))
    }
}

impl Identify for VoidExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for VoidExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _VoidExpr {
    identifier: Identifier,
}

impl _VoidExpr {
    pub fn new(identifier: Identifier) -> _VoidExpr {
        _VoidExpr { identifier: identifier }
    }
}

impl Identify for _VoidExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for _VoidExpr {
    fn typedef(&self) -> Type {
        Type::Primitive(PrimitiveType::Void)
    }
}