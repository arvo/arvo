//! # Abstract Syntax Tree
//!
//! The abstract syntax tree (AST) is the representation of the raw input,
//! after it has been tokenised and parsed. The tokenisation process validates
//! individual keywords and elements, and the parser checks that they are
//! assembled in a way that is grammatically correct. At this stage, no
//! semantic analysis has been done to ensure that the program is actually
//! valid.

use super::identifier::{Identifier, Identify, Symbol, Symbolise};

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


// use super::bair::{Object, PrimitiveType};
// use super::identifier::{Identifier, Identify, Symbol, Symbolise};

// #[macro_use]
// pub mod macros;
// pub mod prelude;

// /// The `Typedef` trait is implemented by nodes that can express some `Type`.
// pub trait Typedef {
//     fn typedef(&self) -> Type;
// }

// /// A `BlockExpr` contains a set of `Exprs` that will be evaluated in a
// /// non-deterministic order. It also contains a return `Expr`.
// #[derive(Clone)]
// pub struct BlockExpr {
//     identifier: Identifier,
//     body: Exprs,
//     ret: Box<Expr>,
// }

// impl BlockExpr {
//     pub fn new(identifier: Identifier, body: Exprs, ret: Expr) -> BlockExpr {
//         BlockExpr {
//             identifier: identifier,
//             body: body,
//             ret: ret,
//         }
//     }

//     pub fn body(&self) -> &Exprs {
//         &self.body
//     }

//     pub fn ret(&self) -> &Expr {
//         &self.ret
//     }
// }

// impl Identify for BlockExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for BlockExpr {
//     fn typedef(&self) -> Type {
//         self.ret().typedef()
//     }
// }

// /// A `CallExpr` is a function call to some target function. The target
// /// function is an `Expr` to allow for higher-order functions to be used as
// /// call targets. It contains a set of argument `Exprs` that will be passed to
// /// the function call.
// #[derive(Clone)]
// pub struct CallExpr {
//     identifier: Identifier,
//     target: Box<Expr>,
//     arguments: Exprs,
// }

// impl CallExpr {
//     pub fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> CallExpr {
//         CallExpr {
//             identifier: identifier,
//             target: target,
//             arguments: arguments,
//         }
//     }

//     pub fn target(&self) -> &Expr {
//         &self.target
//     }

//     pub fn arguments(&self) -> &Exprs {
//         &self.arguments
//     }
// }

// impl Identify for CallExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for CallExpr {
//     fn typedef(&self) -> Type {
//         if let Type::Lambda(lambda_type) = self.target().typedef() {
//             return lambda_type.ret();
//         }
//         unimplemented!()
//     }
// }

// /// A `DefExpr` is an expression that defines a new variable.
// #[derive(Clone)]
// pub struct DefExpr {
//     identifier: Identifier,
//     variable: Variable,
//     definition: Box<Expr>,
// }

// impl DefExpr {
//     pub fn new(identifier: Identifier, variable: Variable, definition: Expr) -> DefExpr {
//         DefExpr {
//             identifier: identifier,
//             variable: variable,
//             definition: definition,
//         }
//     }

//     /// Returns a reference to the Variable that is defined.
//     pub fn variable(&self) -> &Variable {
//         &self.variable
//     }

//     /// Returns a reference to the Expr that is used to define the variable.
//     pub fn definition(&self) -> &Expr {
//         &self.definition
//     }
// }

// impl Identify for DefExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for DefExpr {

//     /// A DefExpr will always return the PrimitiveType::Void type.
//     fn typedef(&self) -> Type {
//         Type::Primitive(PrimitiveType::Void)
//     }
// }

// ///
// #[derive(Clone)]
// pub struct ForExpr(Object<_ForExpr>);

// impl ForExpr {
//     pub fn new(identifier: Identifier,
//                variable: Variable,
//                iterand: Expr,
//                iteration: BlockExpr)
//                -> ForExpr {
//         ForExpr(object!(_ForExpr::new(identifier, variable, iterand, iteration)))
//     }
// }

// impl Identify for ForExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for ForExpr {
//     fn typedef(&self) -> Type {
//         unimplemented!()
//     }
// }

// #[derive(Clone)]
// struct _ForExpr {
//     identifier: Identifier,
//     variable: Variable,
//     iterand: Expr,
//     iteration: BlockExpr,
// }

// impl _ForExpr {
//     fn new(identifier: Identifier,
//            variable: Variable,
//            iterand: Expr,
//            iteration: BlockExpr)
//            -> _ForExpr {
//         _ForExpr {
//             identifier: identifier,
//             variable: variable,
//             iterand: iterand,
//             iteration: iteration,
//         }
//     }
// }

// impl Identify for _ForExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct Function(Object<_Function>);

// impl Function {
//     pub fn new(symbol: Symbol, formals: Variables, ret: Type, body: Option<Expr>) -> Function {
//         Function(object!(_Function::new(symbol, formals, ret, body)))
//     }
// }

// impl Identify for Function {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Symbolise for Function {
//     fn symbolise(&self) -> Symbol {
//         object_proxy![self.0 => symbolise()]
//     }
// }

// impl Typedef for Function {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _Function {
//     symbol: Symbol,
//     formals: Variables,
//     ret: Type,
//     body: Option<Expr>,
// }

// impl _Function {
//     fn new(symbol: Symbol, formals: Variables, ret: Type, body: Option<Expr>) -> _Function {
//         _Function {
//             symbol: symbol,
//             formals: formals,
//             ret: ret,
//             body: body,
//         }
//     }
// }

// impl Identify for _Function {
//     fn identify(&self) -> Identifier {
//         self.symbolise().identify()
//     }
// }

// impl Symbolise for _Function {
//     fn symbolise(&self) -> Symbol {
//         self.symbol.clone()
//     }
// }

// impl Typedef for _Function {
//     fn typedef(&self) -> Type {
//         Type::Lambda(LambdaType::new(self.formals
//                                          .iter()
//                                          .map(|formal| formal.typedef())
//                                          .collect(),
//                                      self.ret.clone()))
//     }
// }

// ///
// #[derive(Clone)]
// pub struct IfExpr(Object<_IfExpr>);

// impl IfExpr {
//     pub fn new(identifier: Identifier,
//                conditional_expr: Expr,
//                then_block: BlockExpr,
//                else_block: BlockExpr)
//                -> IfExpr {
//         IfExpr(object!(_IfExpr::new(identifier, conditional_expr, then_block, else_block)))
//     }
// }

// impl Identify for IfExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for IfExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => then_block().typedef()]
//     }
// }

// #[derive(Clone)]
// struct _IfExpr {
//     identifier: Identifier,
//     conditional_expr: Expr,
//     then_block: BlockExpr,
//     else_block: BlockExpr,
// }

// impl _IfExpr {
//     fn new(identifier: Identifier,
//            conditional_expr: Expr,
//            then_block: BlockExpr,
//            else_block: BlockExpr)
//            -> _IfExpr {
//         _IfExpr {
//             identifier: identifier,
//             conditional_expr: conditional_expr,
//             then_block: then_block,
//             else_block: else_block,
//         }
//     }

//     fn then_block(&self) -> &BlockExpr {
//         &self.then_block
//     }
// }

// impl Identify for _IfExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct ItemExpr(Object<_ItemExpr>);

// impl ItemExpr {
//     pub fn new(identifier: Identifier, item: Item) -> ItemExpr {
//         ItemExpr(object!(_ItemExpr::new(identifier, item)))
//     }
// }

// impl Identify for ItemExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Symbolise for ItemExpr {
//     fn symbolise(&self) -> Symbol {
//         object_proxy![self.0 => symbolise()]
//     }
// }

// impl Typedef for ItemExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _ItemExpr {
//     identifier: Identifier,
//     item: Item,
// }

// impl _ItemExpr {
//     fn new(identifier: Identifier, item: Item) -> _ItemExpr {
//         _ItemExpr {
//             identifier: identifier,
//             item: item,
//         }
//     }
// }

// impl Identify for _ItemExpr {
//     fn identify(&self) -> Identifier {
//         item_proxy![*&self.item => identify()]
//     }
// }

// impl Symbolise for _ItemExpr {
//     fn symbolise(&self) -> Symbol {
//         item_proxy![*&self.item => symbolise()]
//     }
// }

// impl Typedef for _ItemExpr {
//     fn typedef(&self) -> Type {
//         item_proxy![*&self.item => typedef()]
//     }
// }

// #[derive(Clone)]
// pub enum Item {
//     Function(Function),
//     Unresolved(Unresolved),
//     Variable(Variable),
// }

// ///
// #[derive(Clone)]
// pub struct LambdaType(Object<_LambdaType>);

// impl LambdaType {
//     pub fn new(formals: Types, ret: Type) -> LambdaType {
//         LambdaType(object!(_LambdaType::new(formals, ret)))
//     }

//     pub fn ret(&self) -> Type {
//         object_proxy![self.0 => ret().clone()]
//     }
// }

// #[derive(Clone)]
// struct _LambdaType {
//     formals: Types,
//     ret: Type,
// }

// impl _LambdaType {
//     fn new(formals: Types, ret: Type) -> _LambdaType {
//         _LambdaType {
//             formals: formals,
//             ret: ret,
//         }
//     }

//     fn ret(&self) -> &Type {
//         &self.ret
//     }
// }

// ///
// #[derive(Clone)]
// pub struct LiteralExpr(Object<_LiteralExpr>);

// impl Identify for LiteralExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for LiteralExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _LiteralExpr {
//     identifier: Identifier,
//     literal: Literal,
// }

// impl Identify for _LiteralExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for _LiteralExpr {
//     fn typedef(&self) -> Type {
//         self.literal.typedef()
//     }
// }

// #[derive(Clone)]
// pub enum Literal {
//     Bool(bool),
//     Char(char),
//     F32(f32),
//     F64(i64),
//     I8(i8),
//     I16(i16),
//     I32(i32),
//     I64(i64),
//     Str(String),
//     U8(u8),
//     U16(u16),
//     U32(u32),
//     U64(u64),
//     USize(usize),
// }

// impl Typedef for Literal {
//     fn typedef(&self) -> Type {
//         match *self {
//             Literal::Bool(..) => Type::Primitive(PrimitiveType::Bool),
//             Literal::Char(..) => Type::Primitive(PrimitiveType::Char),
//             Literal::F32(..) => Type::Primitive(PrimitiveType::F32),
//             Literal::F64(..) => Type::Primitive(PrimitiveType::F64),
//             Literal::I8(..) => Type::Primitive(PrimitiveType::I8),
//             Literal::I16(..) => Type::Primitive(PrimitiveType::I16),
//             Literal::I32(..) => Type::Primitive(PrimitiveType::I32),
//             Literal::I64(..) => Type::Primitive(PrimitiveType::I64),
//             Literal::Str(..) => Type::Primitive(PrimitiveType::Str),
//             Literal::U8(..) => Type::Primitive(PrimitiveType::U8),
//             Literal::U16(..) => Type::Primitive(PrimitiveType::U16),
//             Literal::U32(..) => Type::Primitive(PrimitiveType::U32),
//             Literal::U64(..) => Type::Primitive(PrimitiveType::U64),
//             Literal::USize(..) => Type::Primitive(PrimitiveType::USize),
//         }
//     }
// }

// ///
// #[derive(Clone)]
// pub struct Module(Object<_Module>);

// impl Module {
//     pub fn new(symbol: Symbol, decls: Decls) -> Module {
//         Module(object!(_Module::new(symbol, decls)))
//     }
// }

// impl Identify for Module {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Symbolise for Module {
//     fn symbolise(&self) -> Symbol {
//         object_proxy![self.0 => symbolise()]
//     }
// }

// #[derive(Clone)]
// struct _Module {
//     symbol: Symbol,
//     decls: Decls,
// }

// impl _Module {
//     fn new(symbol: Symbol, decls: Decls) -> _Module {
//         _Module {
//             symbol: symbol,
//             decls: decls,
//         }
//     }
// }

// impl Identify for _Module {
//     fn identify(&self) -> Identifier {
//         self.symbol.identify()
//     }
// }

// impl Symbolise for _Module {
//     fn symbolise(&self) -> Symbol {
//         self.symbol.clone()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct RefExpr(Object<_RefExpr>);

// impl RefExpr {
//     pub fn new(identifier: Identifier, referent: Expr) -> RefExpr {
//         RefExpr(object!(_RefExpr::new(identifier, referent)))
//     }
// }

// impl Identify for RefExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for RefExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _RefExpr {
//     identifier: Identifier,
//     referent: Expr,
// }

// impl _RefExpr {
//     fn new(identifier: Identifier, referent: Expr) -> _RefExpr {
//         _RefExpr {
//             identifier: identifier,
//             referent: referent,
//         }
//     }
// }

// impl Identify for _RefExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for _RefExpr {
//     fn typedef(&self) -> Type {
//         Type::Ref(RefType::new(self.referent.typedef()))
//     }
// }

// ///
// #[derive(Clone)]
// pub struct RefType(Object<_RefType>);

// impl RefType {
//     pub fn new(referent: Type) -> RefType {
//         RefType(object!(_RefType::new(referent)))
//     }
// }

// #[derive(Clone)]
// struct _RefType {
//     referent: Type,
// }

// impl _RefType {
//     fn new(referent: Type) -> _RefType {
//         _RefType { referent: referent }
//     }
// }

// ///
// #[derive(Clone)]
// pub struct StructExpr(Object<_StructExpr>);

// impl StructExpr {
//     pub fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> StructExpr {
//         StructExpr(object!(_StructExpr::new(identifier, elements, ty)))
//     }
// }

// impl Identify for StructExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for StructExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _StructExpr {
//     identifier: Identifier,
//     elements: Exprs,
//     ty: StructType,
// }

// impl _StructExpr {
//     fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> _StructExpr {
//         _StructExpr {
//             identifier: identifier,
//             elements: elements,
//             ty: ty,
//         }
//     }
// }

// impl Identify for _StructExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for _StructExpr {
//     fn typedef(&self) -> Type {
//         Type::Struct(self.ty.clone())
//     }
// }

// ///
// #[derive(Clone)]
// pub struct StructElementExpr(Object<_StructElementExpr>);

// impl StructElementExpr {
//     pub fn new(identifier: Identifier, target: Expr, variable: Variable) -> StructElementExpr {
//         StructElementExpr(object!(_StructElementExpr::new(identifier, target, variable)))
//     }
// }

// impl Identify for StructElementExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for StructElementExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _StructElementExpr {
//     identifier: Identifier,
//     target: Expr,
//     variable: Variable,
// }

// impl _StructElementExpr {
//     fn new(identifier: Identifier, target: Expr, variable: Variable) -> _StructElementExpr {
//         _StructElementExpr {
//             identifier: identifier,
//             target: target,
//             variable: variable,
//         }
//     }
// }

// impl Identify for _StructElementExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for _StructElementExpr {
//     fn typedef(&self) -> Type {
//         self.variable.typedef()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct StructType(Object<_StructType>);

// impl StructType {
//     pub fn new(elements: Variables) -> StructType {
//         StructType(object!(_StructType::new(elements)))
//     }

//     pub fn for_elements<T, F: FnMut(&mut Variable) -> T>(&self, mut f: F) {
//         object_proxy_mut![self.0 => elements_mut().iter_mut() loop element {
//             f(element);
//         }]
//     }

//     pub fn map_elements<T, F: FnMut(&mut Variable) -> T>(&self, f: F) -> Vec<T> {
//         object_proxy_mut![self.0 => elements_mut().iter_mut().map(f).collect()]
//     }

//     pub fn num_elements(&self) -> usize {
//         object_proxy![self.0 => elements().len()]
//     }
// }

// #[derive(Clone)]
// struct _StructType {
//     elements: Variables,
// }

// impl _StructType {
//     fn new(elements: Variables) -> _StructType {
//         _StructType { elements: elements }
//     }

//     fn elements(&self) -> &Variables {
//         &self.elements
//     }

//     fn elements_mut(&mut self) -> &mut Variables {
//         &mut self.elements
//     }
// }

// ///
// #[derive(Clone)]
// pub enum Type {
//     Lambda(LambdaType),
//     Primitive(PrimitiveType),
//     Ref(RefType),
//     Struct(StructType),
//     Unresolved(UnresolvedType),
// }

// pub type Types = Vec<Type>;

// ///
// #[derive(Clone)]
// pub struct Unresolved(Object<_Unresolved>);

// impl Identify for Unresolved {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Symbolise for Unresolved {
//     fn symbolise(&self) -> Symbol {
//         object_proxy![self.0 => symbolise()]
//     }
// }

// impl Typedef for Unresolved {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// struct _Unresolved {
//     symbol: Symbol,
// }

// impl Identify for _Unresolved {
//     fn identify(&self) -> Identifier {
//         self.symbolise().identify()
//     }
// }

// impl Symbolise for _Unresolved {
//     fn symbolise(&self) -> Symbol {
//         self.symbol.clone()
//     }
// }

// impl Typedef for _Unresolved {
//     fn typedef(&self) -> Type {
//         unimplemented!()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct UnresolvedType(Object<_UnresolvedType>);

// impl Symbolise for UnresolvedType {
//     fn symbolise(&self) -> Symbol {
//         object_proxy![self.0 => symbolise()]
//     }
// }

// impl Typedef for UnresolvedType {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// struct _UnresolvedType {
//     symbol: Symbol,
// }

// impl Symbolise for _UnresolvedType {
//     fn symbolise(&self) -> Symbol {
//         self.symbol.clone()
//     }
// }

// impl Typedef for _UnresolvedType {
//     fn typedef(&self) -> Type {
//         unimplemented!()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct Variable(Object<_Variable>);

// impl Variable {
//     pub fn new(symbol: Symbol, ty: Type) -> Variable {
//         Variable(object!(_Variable::new(symbol, ty)))
//     }
// }

// impl Identify for Variable {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Symbolise for Variable {
//     fn symbolise(&self) -> Symbol {
//         object_proxy![self.0 => symbolise()]
//     }
// }

// impl Typedef for Variable {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// pub type Variables = Vec<Variable>;

// #[derive(Clone)]
// struct _Variable {
//     symbol: Symbol,
//     ty: Type,
// }

// impl _Variable {
//     fn new(symbol: Symbol, ty: Type) -> _Variable {
//         _Variable {
//             symbol: symbol,
//             ty: ty,
//         }
//     }
// }

// impl Identify for _Variable {
//     fn identify(&self) -> Identifier {
//         self.symbolise().identify()
//     }
// }

// impl Symbolise for _Variable {
//     fn symbolise(&self) -> Symbol {
//         self.symbol.clone()
//     }
// }

// impl Typedef for _Variable {
//     fn typedef(&self) -> Type {
//         self.ty.clone()
//     }
// }

// ///
// #[derive(Clone)]
// pub struct VoidExpr(Object<_VoidExpr>);

// impl VoidExpr {
//     pub fn new(identifier: Identifier) -> VoidExpr {
//         VoidExpr(object!(_VoidExpr::new(identifier)))
//     }
// }

// impl Identify for VoidExpr {
//     fn identify(&self) -> Identifier {
//         object_proxy![self.0 => identify()]
//     }
// }

// impl Typedef for VoidExpr {
//     fn typedef(&self) -> Type {
//         object_proxy![self.0 => typedef()]
//     }
// }

// #[derive(Clone)]
// struct _VoidExpr {
//     identifier: Identifier,
// }

// impl _VoidExpr {
//     pub fn new(identifier: Identifier) -> _VoidExpr {
//         _VoidExpr { identifier: identifier }
//     }
// }

// impl Identify for _VoidExpr {
//     fn identify(&self) -> Identifier {
//         self.identifier.clone()
//     }
// }

// impl Typedef for _VoidExpr {
//     fn typedef(&self) -> Type {
//         Type::Primitive(PrimitiveType::Void)
//     }
// }

// ///
// #[derive(Clone)]
// pub enum Decl {
//     Function(Function),
//     Module(Module),
//     Type(Type),
// }

// pub type Decls = Vec<Decl>;

// ///
// #[derive(Clone)]
// pub enum Expr {
//     Block(BlockExpr),
//     Call(CallExpr),
//     Def(DefExpr),
//     For(ForExpr),
//     If(IfExpr),
//     Item(ItemExpr),
//     Literal(LiteralExpr),
//     Ref(RefExpr),
//     Struct(StructExpr),
//     StructElement(StructElementExpr),
//     Void(VoidExpr),
// }

// impl Identify for Expr {
//     fn identify(&self) -> Identifier {
//         expr_proxy![*self => identify()]
//     }
// }

// impl Typedef for Expr {
//     fn typedef(&self) -> Type {
//         expr_proxy![*self => typedef()]
//     }
// }

// pub type Exprs = Vec<Expr>;