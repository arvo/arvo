//! # Abstract Syntax Tree
//!
//! The abstract syntax tree (AST) is an intermediate representation of the
//! raw input after it has been tokenised and parsed. The AST may only be
//! partially complete with respect to the raw input, because there may be
//! errors in the raw input.

use super::identifier::Identifier;
use super::lexer::{Span, Spanned};

///
#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    ModuleDecl(ModuleDecl),
    FunctionDecl(FunctionDecl),
    TypeDecl(TypeDecl),
    Expr(Expr),
    Nil,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDecl {
    pub identifier: Identifier,
    pub statements: ModuleStatements,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleStatement {
    pub is_expose: bool,
    pub kind: ModuleStatementKind,
}

impl ModuleStatement {
    pub fn new(is_expose: bool, kind: ModuleStatementKind) -> ModuleStatement {
        ModuleStatement {
            is_expose: is_expose,
            kind: kind,
        }
    }
}

///
pub type ModuleStatements = Vec<ModuleStatement>;

///
#[derive(Clone, Debug, PartialEq)]
pub enum ModuleStatementKind {
    Function(FunctionDecl),
    Type(TypeDecl),
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl {
    pub is_extern: bool,
    pub identifier: Identifier,
    pub func: Function,
}

impl FunctionDecl {
    pub fn new<Ident, Func>(is_extern: bool, identifier: Ident, func: Func) -> FunctionDecl
        where Ident: Into<Identifier>,
              Func: Into<Function>
    {
        FunctionDecl {
            is_extern: is_extern,
            identifier: identifier.into(),
            func: func.into(),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub formals: FunctionFormals,
    pub ret: Type,
    pub body: Option<Expr>,
}

impl Function {
    pub fn new<Formals, Ty>(formals: Formals, ret: Ty, body: Option<Expr>) -> Function
        where Formals: Into<FunctionFormals>,
              Ty: Into<Type>
    {
        Function {
            formals: formals.into(),
            ret: ret.into(),
            body: body,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionFormal {
    pub is_mut: bool,
    pub identifier: Identifier,
    pub ty: Type,
}

impl FunctionFormal {
    pub fn new<Ident, Ty>(is_mut: bool, identifier: Ident, ty: Ty) -> FunctionFormal
        where Ident: Into<Identifier>,
              Ty: Into<Type>
    {
        FunctionFormal {
            is_mut: is_mut,
            identifier: identifier.into(),
            ty: ty.into(),
        }
    }
}

///
pub type FunctionFormals = Vec<FunctionFormal>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl {
    pub identifier: Identifier,
    pub type_params: TypeParams,
    pub kind: TypeDeclKind,
}

///
pub type TypeParam = Identifier;

///
pub type TypeParams = Vec<TypeParam>;

///
#[derive(Clone, Debug, PartialEq)]
pub enum TypeDeclKind {
    Enum(EnumDecl),
    Struct(StructDecl),
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct EnumDecl {
    pub identifier: Identifier,
    pub params: TypeParams,
    pub variants: EnumVariants,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum EnumVariant {
    EnumTuple(EnumTupleVariant),
    EnumStruct(EnumStructVariant),
}

///
pub type EnumVariants = Vec<EnumVariant>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct EnumTupleVariant {
    pub identifier: Identifier,
    pub fields: EnumTupleVariantFields,
}

///
pub type EnumTupleVariantField = Type;

///
pub type EnumTupleVariantFields = Vec<EnumTupleVariantField>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct EnumStructVariant {
    pub identifier: Identifier,
    pub fields: EnumStructVariantFields,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct EnumStructVariantField {
    pub identifier: Identifier,
    pub ty: Type,
}

///
pub type EnumStructVariantFields = Vec<EnumStructVariantField>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub identifier: Identifier,
    pub params: TypeParams,
    pub fields: StructFields,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct StructFields {
    pub is_expose: bool,
    pub identifier: Identifier,
    pub ty: Type,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Channel(Box<ChannelType>),
    List(Box<ListType>),
    Optional(Box<OptionalType>),
    Ref(Box<Type>),
    RefMut(Box<Type>),
    Tuple(Box<TupleType>),
    Unresolved(Box<UnresolvedType>),
}

/// A `Type` can be created from an `UnresolvedType`.
impl<Ty: Into<Box<UnresolvedType>>> From<Ty> for Type {
    fn from(ty: Ty) -> Type {
        Type::Unresolved(ty.into())
    }
}

///
pub type Types = Vec<Type>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelType {
    pub generic_type: Type,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ListType {
    pub generic_type: Type,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct OptionalType {
    pub generic_type: Type,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct TupleType {
    pub generic_types: Types,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct UnresolvedType {
    pub identifier: Identifier,
    pub generic_types: Types,
}

impl UnresolvedType {
    pub fn new<Ident, Tys>(identifier: Ident, generic_types: Tys) -> UnresolvedType
        where Ident: Into<Identifier>,
              Tys: Into<Types>
    {
        UnresolvedType {
            identifier: identifier.into(),
            generic_types: generic_types.into(),
        }
    }
    pub fn string() -> UnresolvedType {
        UnresolvedType::new("string", Types::new())
    }

    pub fn void() -> UnresolvedType {
        UnresolvedType::new("void", Types::new())
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Box<AssignExpr>),
    Block(Box<BlockExpr>),
    Call(Box<CallExpr>),
    Channel(Box<ChannelExpr>),
    Deref(Box<DerefExpr>),
    For(Box<ForExpr>),
    If(Box<IfExpr>),
    Item(Box<ItemExpr>),
    List(Box<ListExpr>),
    Literal(Box<LiteralExpr>),
    Operator(Box<OperatorExpr>),
    Ref(Box<RefExpr>),
    Select(Box<SelectExpr>),
    Tuple(Box<TupleExpr>),
    Void(Box<VoidExpr>),
}

impl Spanned for Expr {
    fn span(&self) -> &Span {
        match *self {
            Expr::Literal(ref expr) => expr.span(),
            _ => unimplemented!(),
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        match *self {
            Expr::Literal(ref mut expr) => expr.span_mut(),
            _ => unimplemented!(),
        }
    }
}

/// An `Expr` can be created from a `BinaryOperatorExpr`.
impl From<BinaryOperatorExpr> for Expr {
    fn from(expr: BinaryOperatorExpr) -> Expr {
        Expr::Operator(Box::new(OperatorExpr::Binary(expr.into())))
    }
}

/// An `Expr` can be created from a `Box<BlockExpr>`.
impl From<Box<BinaryOperatorExpr>> for Expr {
    fn from(expr: Box<BinaryOperatorExpr>) -> Expr {
        Expr::Operator(Box::new(OperatorExpr::Binary(expr)))
    }
}

/// An `Expr` can be created from a `BlockExpr`.
impl From<BlockExpr> for Expr {
    fn from(expr: BlockExpr) -> Expr {
        Expr::Block(expr.into())
    }
}

/// An `Expr` can be created from a `Box<BlockExpr>`.
impl From<Box<BlockExpr>> for Expr {
    fn from(expr: Box<BlockExpr>) -> Expr {
        Expr::Block(expr)
    }
}

/// An `Expr` can be created from a `LiteralExpr`.
impl From<LiteralExpr> for Expr {
    fn from(expr: LiteralExpr) -> Expr {
        Expr::Literal(expr.into())
    }
}

/// An `Expr` can be created from a `Box<LiteralExpr>`.
impl From<Box<LiteralExpr>> for Expr {
    fn from(expr: Box<LiteralExpr>) -> Expr {
        Expr::Literal(expr)
    }
}

/// An `Expr` can be created from a `VoidExpr`.
impl From<VoidExpr> for Expr {
    fn from(expr: VoidExpr) -> Expr {
        Expr::Void(expr.into())
    }
}

/// An `Expr` can be created from a `Box<VoidExpr>`.
impl From<Box<VoidExpr>> for Expr {
    fn from(expr: Box<VoidExpr>) -> Expr {
        Expr::Void(expr)
    }
}

///
pub type Exprs = Vec<Expr>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct AssignExpr {
    pub lhs: Pattern,
    pub rhs: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct BlockExpr {
    pub statements: BlockStatements,
    pub ret: Expr,
}

impl BlockExpr {
    pub fn new<Stmts, Ret>(statements: Stmts, ret: Ret) -> BlockExpr
        where Stmts: Into<BlockStatements>,
              Ret: Into<Expr>
    {
        BlockExpr {
            statements: statements.into(),
            ret: ret.into(),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum BlockStatement {
    Let(Box<LetStatement>),
    Expr(Box<Expr>),
}

///
pub type BlockStatements = Vec<BlockStatement>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct LetStatement {
    pub is_mut: bool,
    pub lhs: Pattern,
    pub ty: Type,
    pub rhs: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct CallExpr {
    pub target: Expr,
    pub arguments: Exprs,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelExpr {
    pub begin: Option<Expr>,
    pub end: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct DerefExpr {
    pub dereferent: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ForExpr {}

///
#[derive(Clone, Debug, PartialEq)]
pub struct IfExpr {
    pub condition: Expr,
    pub then_block: BlockExpr,
    pub else_block: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ItemExpr {
    pub path: Expr,
    pub item: Identifier,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ListExpr {
    pub items: Exprs,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum LiteralExpr {
    Bool(bool, Span),
    Char(char, Span),
    Float(f64, Span),
    Int(i64, Span),
    Str(String, Span),
}

impl Spanned for LiteralExpr {
    fn span(&self) -> &Span {
        use self::LiteralExpr::*;

        match *self {
            Bool(_, ref span, ..) => span,
            Char(_, ref span, ..) => span,
            Float(_, ref span, ..) => span,
            Int(_, ref span, ..) => span,
            Str(_, ref span, ..) => span,
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        use self::LiteralExpr::*;

        match *self {
            Bool(_, ref mut span, ..) => span,
            Char(_, ref mut span, ..) => span,
            Float(_, ref mut span, ..) => span,
            Int(_, ref mut span, ..) => span,
            Str(_, ref mut span, ..) => span,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum OperatorExpr {
    Binary(Box<BinaryOperatorExpr>),
    Prefix(Box<PrefixOperatorExpr>),
    Suffix(Box<SuffixOperatorExpr>),
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOperatorExpr {
    pub operator: Operator,
    pub lhs: Expr,
    pub rhs: Expr,
}

impl BinaryOperatorExpr {
    pub fn new<Op, LHS, RHS>(operator: Op, lhs: LHS, rhs: RHS) -> BinaryOperatorExpr
        where Op: Into<Operator>,
              LHS: Into<Expr>,
              RHS: Into<Expr>
    {
        BinaryOperatorExpr {
            operator: operator.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct PrefixOperatorExpr {
    pub operator: Operator,
    pub rhs: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct SuffixOperatorExpr {
    pub operator: Operator,
    pub lhs: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    AddEq,
    And,
    Assign,
    Div,
    DivEq,
    Equal,
    GreaterThan,
    GreaterThanEq,
    LessThan,
    LessThanEq,
    Mul,
    MulEq,
    Or,
    PushPop,
    Sub,
    SubEq,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct RefExpr {
    pub referent: Expr,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct SelectExpr {
    pub guards: SelectGuards,
    pub else_block: Option<BlockExpr>,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum SelectGuard {
    Read(SelectReadGuard),
    Write(SelectWriteGuard),
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct SelectReadGuard {
    pub rhs: Expr,
    pub alias: Option<Identifier>,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct SelectWriteGuard {
    pub lhs: Expr,
    pub rhs: Expr,
}

///
pub type SelectGuards = Vec<SelectGuard>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct TupleExpr {
    pub fields: Exprs,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct VoidExpr {
    span: Span,
}

impl VoidExpr {
    pub fn new<S>(span: S) -> VoidExpr
        where S: Into<Span>
    {
        VoidExpr { span: span.into() }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    List(Box<ListPattern>),
    Tuple(Box<TuplePattern>),
    Variable(Box<VariablePattern>),
}

///
pub type Patterns = Vec<Pattern>;

///
#[derive(Clone, Debug, PartialEq)]
pub enum ListPattern {
    Cons(ConsListPattern),
    Enum(EnumListPattern),
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct ConsListPattern {
    pub head: Pattern,
    pub tail: Pattern,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct EnumListPattern {
    pub patterns: Patterns,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct TuplePattern {
    pub fields: Patterns,
}

///
pub type VariablePattern = Identifier;