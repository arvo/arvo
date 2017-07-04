//! # NoIR
//!
//! The normalised intermediate representation ([NoIR](libarvo.noir)) is the normalised form
//! of the AIR. NoIR is produced by normalising an AIR that has been through
//! all necessary compilation passes, and so by the time a program has reached
//! this stage of compilation, it is valid. Optimisation passes, and code
//! generation passes, are done using NoIR.

use super::identifier::{Identifier, Identify, Symbol, Symbolise};

use std::collections::HashMap;

#[macro_use]
pub mod macros;
pub mod context;
pub mod prelude;
pub mod runtime;

pub use self::context::*;

///
#[derive(Clone)]
pub struct AliasType {
    symbol: Symbol,
    inner: Type,
}

///
#[derive(Clone)]
pub struct AssignExpr {
    lhs: RefExpr,
    rhs: Expr,
}

///
#[derive(Clone)]
pub struct BlockExpr {
    pub identifier: Identifier,
    pub prelude: Exprs,
    pub body: Exprs,
    pub epilogue: Exprs,
    pub ret: Expr,
    pub function_table: FunctionTable,
    pub module_table: ModuleTable,
    pub type_table: TypeTable,
}

impl BlockExpr {
    pub fn new(identifier: Identifier,
               prelude: Exprs,
               body: Exprs,
               epilogue: Exprs,
               ret: Expr,
               function_table: FunctionTable,
               module_table: ModuleTable,
               type_table: TypeTable)
               -> BlockExpr {
        BlockExpr {
            identifier: identifier,
            prelude: prelude,
            body: body,
            epilogue: epilogue,
            ret: ret,
            function_table: function_table,
            module_table: module_table,
            type_table: type_table,
        }
    }
}

impl Identify for BlockExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
pub type BlockExprs = Vec<BlockExpr>;

///
#[derive(Clone)]
pub struct CallExpr {
    pub identifier: Identifier,
    pub target: Expr,
    pub arguments: Exprs,
}

impl CallExpr {
    pub fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> CallExpr {
        CallExpr {
            identifier: identifier,
            target: target,
            arguments: arguments,
        }
    }
}

impl Identify for CallExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct DerefExpr {
    identifier: Identifier,
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
    identifier: Identifier,
    formals: Variables,
    iterator: Expr,
    iteration: Box<BlockExpr>,
}

#[derive(Clone)]
struct _ForExpr {
    identifier: Identifier,
    variables: Variables,
    
}

///
#[derive(Clone)]
pub struct Function {
    pub symbol: Symbol,
    pub formals: Variables,
    pub ret: Type,
    pub body: Option<BlockExpr>,
}

impl Function {
    pub fn new(symbol: Symbol, formals: Variables, ret: Type, body: Option<BlockExpr>) -> Function {
        Function {
            symbol: symbol,
            formals: formals,
            ret: ret,
            body: body,
        }
    }
}

impl Identify for Function {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for Function {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
pub type Functions = Vec<Function>;

///
pub type FunctionTable = HashMap<Identifier, Function>;

///
#[derive(Clone)]
pub struct IfExpr {
    identifier: Identifier,
    condition: Expr,
    then_block: Box<BlockExpr>,
    else_block: Box<BlockExpr>,
}

///
#[derive(Clone)]
pub struct ItemExpr {
    identifier: Identifier,
    item: Item,
}

///
#[derive(Clone)]
pub struct LambdaType {
    formals: Types,
    ret: Type,
}

impl LambdaType {
    pub fn new(formals: Types, ret: Type) -> LambdaType {
        LambdaType {
            formals: formals,
            ret: ret,
        }
    }
}

///
#[derive(Clone)]
pub struct LetExpr {
    pub identifier: Identifier,
    pub variable: Variable,
    pub definition: AssignExpr,
}

///
#[derive(Clone)]
pub struct LiteralExpr {
    pub identifier: Identifier,
    pub literal: Literal,
}

///
#[derive(Clone)]
pub struct Module {
    pub symbol: Symbol,
    pub function_table: FunctionTable,
    pub module_table: ModuleTable,
    pub type_table: TypeTable,
}

impl Module {
    pub fn new<S, F, M, T>(symbol: S, function_table: F, module_table: M, type_table: T) -> Module
        where S: Into<Symbol>,
              F: Into<FunctionTable>,
              M: Into<ModuleTable>,
              T: Into<TypeTable>
    {
        Module {
            symbol: symbol.into(),
            function_table: function_table.into(),
            module_table: module_table.into(),
            type_table: type_table.into(),
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
pub type Modules = Vec<Module>;

///
pub type ModuleTable = HashMap<Identifier, Module>;

///
#[derive(Clone)]
pub struct ProcessExpr {
    pub identifier: Identifier,
    pub body: Box<BlockExpr>,
}

impl Identify for ProcessExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct ProcessJoinExpr {
    pub identifier: Identifier,
    pub process_expr: Box<ProcessExpr>,
}

impl Identify for ProcessJoinExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct PtrType {
    inner: Type,
}

impl PtrType {
    fn new(inner: Type) -> PtrType {
        PtrType { inner: inner }
    }
}

///
#[derive(Clone)]
pub struct RefType {
    inner: Type,
}

///
#[derive(Clone)]
pub struct RefExpr {
    identifier: Identifier,
    inner: Expr,
}

///
#[derive(Clone)]
pub struct StructExpr {
    elements: Vec<(Variable, Expr)>,
    ty: Type,
}

///
#[derive(Clone)]
pub struct StructType {
    pub symbol: Symbol,
    pub params: Types,
    pub elements: Variables,
}

///
#[derive(Clone)]
pub struct Variable {
    pub symbol: Symbol,
    pub ty: Type,
}

impl Variable {
    pub fn new(symbol: Symbol, ty: Type) -> Variable {
        Variable {
            symbol: symbol,
            ty: ty,
        }
    }
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
pub struct VariableExpr {
    variable: Variable,
    parent: Exprs,
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

///
#[derive(Clone)]
pub enum Expr {
    Assign(Box<AssignExpr>),
    Block(Box<BlockExpr>),
    Call(Box<CallExpr>),
    Deref(Box<DerefExpr>),
    For(Box<ForExpr>),
    If(Box<IfExpr>),
    Item(Box<ItemExpr>),
    Let(Box<LetExpr>),
    Literal(Box<LiteralExpr>),
    Process(Box<ProcessExpr>),
    ProcessJoin(Box<ProcessJoinExpr>),
    Struct(Box<StructExpr>),
    Ref(Box<RefExpr>),
    Variable(Box<VariableExpr>),
    Void(Box<VoidExpr>),
}

impl From<AssignExpr> for Expr {
    fn from(assign_expr: AssignExpr) -> Expr {
        Expr::Assign(assign_expr.into())
    }
}

impl From<BlockExpr> for Expr {
    fn from(block_expr: BlockExpr) -> Expr {
        Expr::Block(block_expr.into())
    }
}

impl From<CallExpr> for Expr {
    fn from(call_expr: CallExpr) -> Expr {
        Expr::Call(call_expr.into())
    }
}

impl From<VoidExpr> for Expr {
    fn from(void_expr: VoidExpr) -> Expr {
        Expr::Void(void_expr.into())
    }
}

///
pub type Exprs = Vec<Expr>;

///
#[derive(Clone)]
pub enum Item {
    Function(Box<Function>),
    Module(Box<Module>),
    Type(Box<Type>),
    Variable(Box<Variable>),
}

impl From<Function> for Item {
    fn from(item: Function) -> Item {
        Item::Function(item.into())
    }
}

impl From<Module> for Item {
    fn from(item: Module) -> Item {
        Item::Module(item.into())
    }
}

impl From<Type> for Item {
    fn from(item: Type) -> Item {
        Item::Type(item.into())
    }
}

impl From<Variable> for Item {
    fn from(item: Variable) -> Item {
        Item::Variable(item.into())
    }
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
}

impl From<LambdaType> for Type {
    fn from(ty: LambdaType) -> Type {
        Type::Lambda(ty.into())
    }
}

impl From<PrimitiveType> for Type {
    fn from(ty: PrimitiveType) -> Type {
        Type::Primitive(ty.into())
    }
}

impl From<PtrType> for Type {
    fn from(ty: PtrType) -> Type {
        Type::Ptr(ty.into())
    }
}

///
pub type Types = Vec<Type>;

///
pub type TypeTable = HashMap<Identifier, Type>;