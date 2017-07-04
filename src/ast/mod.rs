//! # Abstract Syntax Tree
//!
//! The abstract syntax tree (AST) is the representation of the raw input,
//! after it has been tokenised and parsed. The tokenisation process validates
//! individual keywords and elements, and the parser checks that they are
//! assembled in a way that is grammatically correct. At this stage, no
//! semantic analysis has been done to ensure that the program is actually
//! valid.

use super::identifier::{Identifier, Identify, Symbol, Symbolise};

#[macro_use]
pub mod macros;

///
#[derive(Clone)]
pub struct AliasType {
    inner: Type,
}

///
#[derive(Clone)]
pub struct AssignExpr {
    lhs: Expr,
    rhs: Expr,
}

///
#[derive(Clone)]
pub struct BlockExpr {
    pub body: Exprs,
    pub ret: Expr,
}

///
#[derive(Clone)]
pub struct CallExpr {
    pub target: Expr,
    pub arguments: Exprs,
}

///
#[derive(Clone)]
pub struct DerefExpr {
    inner: Expr,
}

///
#[derive(Clone)]
pub struct EnumType {
    symbol: Symbol,
    params: Types,
    variants: Box<StructType>,
}

///
#[derive(Clone)]
pub struct ForExpr {
    formals: Variables,
    iterator: Expr,
    iteration: Box<BlockExpr>,
}

///
#[derive(Clone)]
pub struct Function {
    pub profile: Box<FunctionProfile>,
    pub body: Expr,
}

///
#[derive(Clone)]
pub struct FunctionProfile {
    pub symbol: Symbol,
    pub formals: Variables,
    pub ret: Type,
}

impl Symbolise for FunctionProfile {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct IfExpr {
    condition: Expr,
    then_block: Box<BlockExpr>,
    else_block: Box<BlockExpr>,
}

///
#[derive(Clone)]
pub struct ItemPathExpr {
    path: Exprs,
    item: Item,
}

///
#[derive(Clone)]
pub struct LambdaType {
    formals: Types,
    ret: Type,
}

///
#[derive(Clone)]
pub struct LetExpr {
    variable: Variable,
    definition: Expr,
}

///
#[derive(Clone)]
pub struct LiteralExpr {
    literal: Literal,
}

///
#[derive(Clone)]
pub struct Module {
    pub symbol: Symbol,
    pub declarations: Decls,
}

impl Module {
    pub fn new<Sym, Declarations>(symbol: Sym, declarations: Declarations) -> Module
        where Sym: Into<Symbol>,
              Declarations: Into<Decls>
    {
        Module {
            symbol: symbol.into(),
            declarations: declarations.into(),
        }
    }
}

impl Identify for Module {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for Module {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct PtrType {
    inner: Expr,
}

///
#[derive(Clone)]
pub struct RefType {
    inner: Expr,
}

///
#[derive(Clone)]
pub struct RefExpr {
    inner: Expr,
}

///
#[derive(Clone)]
pub struct StructExpr {
    elements: Vec<(Symbol, Expr)>,
    ty: Type,
}

///
#[derive(Clone)]
pub struct StructType {
    symbol: Symbol,
    params: Types,
    elements: Variables,
}

///
#[derive(Clone)]
pub struct Unresolved {
    symbol: Symbol,
}

///
#[derive(Clone)]
pub struct UnresolvedType {
    symbol: Symbol,
    params: Types,
}

///
#[derive(Clone)]
pub struct Variable {
    pub symbol: Symbol,
    pub ty: Type,
}

impl Symbolise for Variable {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
pub type Variables = Vec<Variable>;

///
#[derive(Clone)]
pub struct VoidExpr {}

///
#[derive(Clone)]
pub enum Decl {
    Function(Box<Function>),
    FunctionProfile(Box<FunctionProfile>),
    Module(Box<Module>),
    Type(Box<Type>),
}

///
pub type Decls = Vec<Decl>;

///
#[derive(Clone)]
pub enum Expr {
    Assign(Box<AssignExpr>),
    Block(Box<BlockExpr>),
    Call(Box<CallExpr>),
    Deref(Box<DerefExpr>),
    For(Box<ForExpr>),
    If(Box<IfExpr>),
    ItemPath(Box<ItemPathExpr>),
    Let(Box<LetExpr>),
    Literal(Box<LiteralExpr>),
    Struct(Box<StructExpr>),
    Ref(Box<RefExpr>),
    Void(Box<VoidExpr>),
}

///
pub type Exprs = Vec<Expr>;

///
#[derive(Clone)]
pub enum Item {
    FunctionProfile(Box<FunctionProfile>),
    Module(Box<Module>),
    Type(Box<Type>),
    Unresolve(Box<Unresolved>),
    Variable(Box<Variable>),
}

///
#[derive(Clone)]
pub enum Literal {
    Bool(bool),
    Channel(Expr, Expr),
    Char(char),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    List(Exprs),
    ListRange(Expr, Expr),
    Str(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
}

///
#[derive(Clone)]
pub enum PrimitiveType {
    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    Str,
    U8,
    U16,
    U32,
    U64,
    USize,
    Void,
}

///
#[derive(Clone)]
pub enum Type {
    Alias(Box<AliasType>),
    Enum(Box<EnumType>),
    Lambda(Box<LambdaType>),
    Primitive(Box<PrimitiveType>),
    Ptr(Box<PtrType>),
    Ref(Box<RefType>),
    Struct(Box<StructType>),
    Unresolve(Box<UnresolvedType>),
}

///
pub type Types = Vec<Type>;