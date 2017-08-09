//! # Abstract Syntax Tree
//!
//! The abstract syntax tree (AST) is an intermediate representation of the
//! raw input after it has been tokenised and parsed. The AST may only be
//! partially complete with respect to the raw input, because there may be
//! errors in the raw input.

use super::identifier::Identifier;

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
    pub identifier: Identifier,
    pub kind: ModuleStatementKind,
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
    pub formals: FunctionFormals,
    pub ret: Type,
    pub body: Option<Expr>,
}

impl FunctionDecl {
    pub fn new<Ty>(is_extern: bool,
                   formals: FunctionFormals,
                   ret: Ty,
                   body: Option<Expr>)
                   -> FunctionDecl
        where Ty: Into<Type>
    {
        FunctionDecl {
            is_extern: is_extern,
            formals: formals,
            ret: ret.into(),
            body: body,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionFormal {
    pub identifier: Identifier,
    pub is_mut: bool,
    pub is_ref: bool,
    pub ty: Type,
}

impl FunctionFormal {
    pub fn new<Ident>(identifier: Ident, is_mut: bool, is_ref: bool, ty: Type) -> FunctionFormal
        where Ident: Into<Identifier>
    {
        FunctionFormal {
            identifier: identifier.into(),
            is_mut: is_mut,
            is_ref: is_ref,
            ty: ty,
        }
    }
}

///
pub type FunctionFormals = Vec<FunctionFormal>;

///
#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl {
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
    pub fn string() -> UnresolvedType {
        UnresolvedType {
            identifier: "string".into(),
            generic_types: Types::new(),
        }
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
    Bool(bool),
    Char(char),
    Float(f64),
    Integer(i64),
    Str(String),
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
pub struct VoidExpr {}

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