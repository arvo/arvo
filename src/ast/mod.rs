//! Abstract Syntax Tree

use super::bair::{Object, PrimitiveType};
use super::identifier::{Identifier, Identify, Symbol, Symbolise};

pub mod visitor;

macro_rules! message_expr {
    ($expr: expr => $($method: ident ( $($arg: expr ),* )).+) => (match *$expr {
        Expr::Block(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Call(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Def(ref expr) => expr.$($method($($arg),*)).+,
        Expr::For(ref expr) => expr.$($method($($arg),*)).+,
        Expr::If(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Item(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Literal(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Ref(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Struct(ref expr) => expr.$($method($($arg),*)).+,
        Expr::StructElement(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Void(ref expr) => expr.$($method($($arg),*)).+,
    });
}

macro_rules! message_item {
    ($item: expr => $($method: ident ( $($arg: expr ),* )).+) => (match *$item {
        Item::Function(ref item) => item.$($method($($arg),*)).+,
        Item::Unresolved(ref item) => item.$($method($($arg),*)).+,
        Item::Variable(ref item) => item.$($method($($arg),*)).+,
    });
}

macro_rules! message_type {
    ($ty: expr => $($method: ident ( $($arg: expr ),* )).+) => (match *$ty {
        Type::Lambda(ref ty) => ty.$($method($($arg),*)).+,
        Type::Primitive(ref ty) => ty.$($method($($arg),*)).+,
        Type::Ref(ref ty) => ty.$($method($($arg),*)).+,
        Type::Struct(ref ty) => ty.$($method($($arg),*)).+,
        Type::Unresolved(ref ty) => ty.$($method($($arg),*)).+,
    });
}

/// The `Typedef` trait is implemented by nodes that can express some `Type`.
pub trait Typedef {
    fn typedef(&self) -> Type;
}

/// A `BlockExpr` contains a set of `Exprs` that will be evaluated in a
/// non-deterministic order. It also contains a return `Expr`.
#[derive(Clone)]
pub struct BlockExpr(Object<RawBlockExpr>);

impl BlockExpr {
    pub fn new(identifier: Identifier, body: Exprs, ret: Expr) -> BlockExpr {
        BlockExpr(object!(RawBlockExpr::new(identifier, body, ret)))
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
struct RawBlockExpr {
    identifier: Identifier,
    body: Exprs,
    ret: Expr,
}

impl RawBlockExpr {
    fn new(identifier: Identifier, body: Exprs, ret: Expr) -> RawBlockExpr {
        RawBlockExpr {
            identifier: identifier,
            body: body,
            ret: ret,
        }
    }

    fn ret(&self) -> &Expr {
        &self.ret
    }
}

impl Identify for RawBlockExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RawBlockExpr {
    fn typedef(&self) -> Type {
        self.ret.typedef()
    }
}

/// A `CallExpr` is a function call to some target function. The target
/// function is an `Expr` to allow for higher-order functions to be used as
/// call targets. It contains a set of argument `Exprs` that will be passed to
/// the function call.
#[derive(Clone)]
pub struct CallExpr(Object<RawCallExpr>);

impl CallExpr {
    pub fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> CallExpr {
        CallExpr(object!(RawCallExpr::new(identifier, target, arguments)))
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
struct RawCallExpr {
    identifier: Identifier,
    target: Expr,
    arguments: Exprs,
}

impl RawCallExpr {
    fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> RawCallExpr {
        RawCallExpr {
            identifier: identifier,
            target: target,
            arguments: arguments,
        }
    }
}

impl Identify for RawCallExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RawCallExpr {
    fn typedef(&self) -> Type {
        if let Type::Lambda(lambda_ty) = self.target.typedef() {
            return lambda_ty.ret();
        }
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct DefExpr(Object<RawDefExpr>);

impl DefExpr {}

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
struct RawDefExpr {
    identifier: Identifier,
    variable: Variable,
    definition: Expr,
}

impl Identify for RawDefExpr {
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
        message_expr![self => identify()]
    }
}

impl Typedef for Expr {
    fn typedef(&self) -> Type {
        message_expr![self => typedef()]
    }
}

pub type Exprs = Vec<Expr>;

///
#[derive(Clone)]
pub struct ForExpr(Object<RawForExpr>);

impl ForExpr {}

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
pub struct RawForExpr {
    identifier: Identifier,
}

impl Identify for RawForExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct Function(Object<RawFunction>);

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
struct RawFunction {
    symbol: Symbol,
}

impl Identify for RawFunction {
    fn identify(&self) -> Identifier {
        self.symbolise().identify()
    }
}

impl Symbolise for RawFunction {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for RawFunction {
    fn typedef(&self) -> Type {
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct IfExpr(Object<RawIfExpr>);

impl IfExpr {
    pub fn new(identifier: Identifier,
               conditional_expr: Expr,
               then_expr: Expr,
               else_expr: Expr)
               -> IfExpr {
        IfExpr(object!(RawIfExpr::new(identifier, conditional_expr, then_expr, else_expr)))
    }
}

impl Identify for IfExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Typedef for IfExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => then_expr().typedef()]
    }
}

#[derive(Clone)]
struct RawIfExpr {
    identifier: Identifier,
    conditional_expr: Expr,
    then_expr: Expr,
    else_expr: Expr,
}

impl RawIfExpr {
    fn new(identifier: Identifier,
               conditional_expr: Expr,
               then_expr: Expr,
               else_expr: Expr)
               -> RawIfExpr {
        RawIfExpr {
            identifier: identifier,
            conditional_expr: conditional_expr,
            then_expr: then_expr,
            else_expr: else_expr,
        }
    }

    fn then_expr(&self) -> &Expr {
        &self.then_expr
    }
}

impl Identify for RawIfExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct ItemExpr(Object<RawItemExpr>);

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
pub struct RawItemExpr {
    identifier: Identifier,
    item: Item,
}

impl Identify for RawItemExpr {
    fn identify(&self) -> Identifier {
        message_item![&self.item => identify()]
    }
}

impl Symbolise for RawItemExpr {
    fn symbolise(&self) -> Symbol {
        message_item![&self.item => symbolise()]
    }
}

impl Typedef for RawItemExpr {
    fn typedef(&self) -> Type {
        message_item![&self.item => typedef()]
    }
}

#[derive(Clone)]
pub enum Item {
    Function(Function),
    Unresolved(Unresolved),
    Variable(Variable),
}

///
#[derive(Clone)]
pub struct LambdaType(Object<RawLambdaType>);

impl LambdaType {
    pub fn new(formals: Types, ret: Type) -> LambdaType {
        LambdaType(object!(RawLambdaType::new(formals, ret)))
    }

    pub fn ret(&self) -> Type {
        object_proxy![self.0 => ret().clone()]
    }
}

#[derive(Clone)]
struct RawLambdaType {
    formals: Types,
    ret: Type,
}

impl RawLambdaType {
    fn new(formals: Types, ret: Type) -> RawLambdaType {
        RawLambdaType {
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
pub struct LiteralExpr(Object<RawLiteralExpr>);

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
struct RawLiteralExpr {
    identifier: Identifier,
    literal: Literal,
}

impl Identify for RawLiteralExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RawLiteralExpr {
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
pub struct Module(Object<RawModule>);

pub struct RawModule {
}

///
#[derive(Clone)]
pub struct RefExpr(Object<RawRefExpr>);

impl RefExpr {
    pub fn new(identifier: Identifier, referent: Expr) -> RefExpr {
        RefExpr(object!(RawRefExpr::new(identifier, referent)))
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
struct RawRefExpr {
    identifier: Identifier,
    referent: Expr,
}

impl RawRefExpr {
    fn new(identifier: Identifier, referent: Expr) -> RawRefExpr {
        RawRefExpr {
            identifier: identifier,
            referent: referent,
        }
    }
}

impl Identify for RawRefExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RawRefExpr {
    fn typedef(&self) -> Type {
        Type::Ref(RefType::new(self.referent.typedef()))
    }
}

///
#[derive(Clone)]
pub struct RefType(Object<RawRefType>);

impl RefType {
    pub fn new(referent: Type) -> RefType {
        RefType(object!(RawRefType::new(referent)))
    }
}

#[derive(Clone)]
struct RawRefType {
    referent: Type,
}

impl RawRefType {
    fn new(referent: Type) -> RawRefType {
        RawRefType {
            referent: referent,
        }
    }
}

///
#[derive(Clone)]
pub struct StructExpr(Object<RawStructExpr>);

impl StructExpr {
    pub fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> StructExpr {
        StructExpr(object!(RawStructExpr::new(identifier, elements, ty)))
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
struct RawStructExpr {
    identifier: Identifier,
    elements: Exprs,
    ty: StructType,
}

impl RawStructExpr {
    fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> RawStructExpr {
        RawStructExpr {
            identifier: identifier,
            elements: elements,
            ty: ty,
        }
    }
}

impl Identify for RawStructExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RawStructExpr {
    fn typedef(&self) -> Type {
        Type::Struct(self.ty.clone())
    }
}

///
#[derive(Clone)]
pub struct StructElementExpr(Object<RawStructElementExpr>);

impl StructElementExpr {
    pub fn new(identifier: Identifier, target: Expr, variable: Variable) -> StructElementExpr {
        StructElementExpr(object!(RawStructElementExpr::new(identifier, target, variable)))
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
struct RawStructElementExpr {
    identifier: Identifier,
    target: Expr,
    variable: Variable,
}

impl RawStructElementExpr {
    fn new(identifier: Identifier, target: Expr, variable: Variable) -> RawStructElementExpr {
        RawStructElementExpr {
            identifier: identifier,
            target: target,
            variable: variable,
        }
    }
}

impl Identify for RawStructElementExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RawStructElementExpr {
    fn typedef(&self) -> Type {
        self.variable.typedef()
    }
}

///
#[derive(Clone)]
pub struct StructType(Object<RawStructType>);

impl StructType {
    pub fn new(elements: Variables) -> StructType {
        StructType(object!(RawStructType::new(elements)))
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
struct RawStructType {
    elements: Variables,
}

impl RawStructType {
    fn new(elements: Variables) -> RawStructType {
        RawStructType { elements: elements }
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
pub struct Unresolved(Object<RawUnresolved>);

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

struct RawUnresolved {
    symbol: Symbol,
}

impl Identify for RawUnresolved {
    fn identify(&self) -> Identifier {
        self.symbolise().identify()
    }
}

impl Symbolise for RawUnresolved {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for RawUnresolved {
    fn typedef(&self) -> Type {
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct UnresolvedType(Object<RawUnresolvedType>);

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

struct RawUnresolvedType {
    symbol: Symbol,
}

impl Symbolise for RawUnresolvedType {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for RawUnresolvedType {
    fn typedef(&self) -> Type {
        unimplemented!()
    }
}

///
#[derive(Clone)]
pub struct Variable(Object<RawVariable>);

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
struct RawVariable {
    symbol: Symbol,
    ty: Type,
}

impl RawVariable {}

impl Identify for RawVariable {
    fn identify(&self) -> Identifier {
        self.symbolise().identify()
    }
}

impl Symbolise for RawVariable {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl Typedef for RawVariable {
    fn typedef(&self) -> Type {
        self.ty.clone()
    }
}

///
#[derive(Clone)]
pub struct VoidExpr {
    identifier: Identifier,
}

impl VoidExpr {
    pub fn new(identifier: Identifier) -> VoidExpr {
        VoidExpr { identifier: identifier }
    }
}

impl Identify for VoidExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for VoidExpr {
    fn typedef(&self) -> Type {
        Type::Primitive(PrimitiveType::Void)
    }
}