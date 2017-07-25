//! # Abstract Syntax Tree
//!
//! The abstract syntax tree (AST) is the representation of the raw input,
//! after it has been tokenised and parsed. The tokenisation process validates
//! individual keywords and elements, and the parser checks that they are
//! assembled in a way that is grammatically correct.

use super::identifier::Identifier;

///
#[derive(Clone)]
pub struct ModuleDecl {
    pub identifier: Identifier,
    pub statements: ModuleStatements,
}

///
#[derive(Clone)]
pub struct ModuleStatement {
    pub is_expose: bool,
    pub identifier: Identifier,
    pub kind: ModuleStatementKind,
}

///
pub type ModuleStatements = Vec<ModuleStatement>;

///
#[derive(Clone)]
pub enum ModuleStatementKind {
    Function(FunctionDecl),
    Type(TypeDecl),
}

///
#[derive(Clone)]
pub struct FunctionDecl {
    pub is_extern: bool,
    pub formals: FunctionFormals,
    pub ret: Type,
    pub body: Option<Expr>,
}

///
#[derive(Clone)]
pub struct FunctionFormal {
    pub identifier: Identifier,
    pub is_mut: bool,
    pub is_ref: bool,
    pub ty: Type,
}

///
pub type FunctionFormals = Vec<FunctionFormal>;

///
#[derive(Clone)]
pub struct TypeDecl {
    pub type_params: TypeParams,
    pub kind: TypeDeclKind,
}

///
pub type TypeParam = Identifier;

///
pub type TypeParams = Vec<TypeParam>;

///
#[derive(Clone)]
pub enum TypeDeclKind {
    Enum(EnumDecl),
    Struct(StructDecl),
}

///
#[derive(Clone)]
pub struct EnumDecl {
    pub identifier: Identifier,
    pub params: TypeParams,
    pub variants: EnumVariants,
}

///
#[derive(Clone)]
pub enum EnumVariant {
    EnumTuple(EnumTupleVariant),
    EnumStruct(EnumStructVariant),
}

///
pub type EnumVariants = Vec<EnumVariant>;

///
#[derive(Clone)]
pub struct EnumTupleVariant {
    pub identifier: Identifier,
    pub fields: EnumTupleVariantFields,
}

///
pub type EnumTupleVariantField = Type;

///
pub type EnumTupleVariantFields = Vec<EnumTupleVariantField>;

///
#[derive(Clone)]
pub struct EnumStructVariant {
    pub identifier: Identifier,
    pub fields: EnumStructVariantFields,
}

///
#[derive(Clone)]
pub struct EnumStructVariantField {
    pub identifier: Identifier,
    pub ty: Type,
}

///
pub type EnumStructVariantFields = Vec<EnumStructVariantField>;

///
#[derive(Clone)]
pub struct StructDecl {
    pub identifier: Identifier,
    pub params: TypeParams,
    pub fields: StructFields,
}

///
#[derive(Clone)]
pub struct StructFields {
    pub is_expose: bool,
    pub identifier: Identifier,
    pub ty: Type,
}

///
#[derive(Clone)]
pub enum Type {
    Channel(Box<ChannelType>),
    List(Box<ListType>),
    Optional(Box<OptionalType>),
    Tuple(Box<TupleType>),
    Unresolved(Box<UnresolvedType>),
}

///
pub type Types = Vec<Type>;

///
#[derive(Clone)]
pub struct ChannelType {
    pub generic_type: Type,
}

///
#[derive(Clone)]
pub struct ListType {
    pub generic_type: Type,
}

///
#[derive(Clone)]
pub struct OptionalType {
    pub generic_type: Type,
}

///
#[derive(Clone)]
pub struct TupleType {
    pub generic_types: Types,
}

///
#[derive(Clone)]
pub struct UnresolvedType {
    pub identifier: Identifier,
    pub generic_types: Types,
}

///
#[derive(Clone)]
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
#[derive(Clone)]
pub struct AssignExpr {
    pub lhs: Pattern,
    pub rhs: Expr,
}

///
#[derive(Clone)]
pub struct BlockExpr {
    pub statements: BlockStatements,
    pub ret: Expr,
}

///
#[derive(Clone)]
pub enum BlockStatement {
    Let(Box<LetStatement>),
    Expr(Box<Expr>),
}

///
pub type BlockStatements = Vec<BlockStatement>;

///
#[derive(Clone)]
pub struct LetStatement {
    pub is_mut: bool,
    pub lhs: Pattern,
    pub ty: Type,
    pub rhs: Expr,
}

///
#[derive(Clone)]
pub struct CallExpr {
    pub target: Expr,
    pub arguments: Exprs,
}

///
#[derive(Clone)]
pub struct ChannelExpr {
    pub begin: Option<Expr>,
    pub end: Expr,
}

///
#[derive(Clone)]
pub struct DerefExpr {
    pub dereferent: Expr,
}

///
#[derive(Clone)]
pub struct ForExpr {
}

///
#[derive(Clone)]
pub struct IfExpr {
    pub condition: Expr,
    pub then_block: BlockExpr,
    pub else_block: Expr,
}

///
#[derive(Clone)]
pub struct ItemExpr {
    pub path: Expr,
    pub item: Identifier,
}

///
#[derive(Clone)]
pub enum LiteralExpr {
    Bool(bool),
    Char(char),
    Float(f64),
    Integer(i64),
    Str(String),
}

///
#[derive(Clone)]
pub enum OperatorExpr {
    Binary(Box<BinaryOperatorExpr>),
    Prefix(Box<PrefixOperatorExpr>),
    Suffix(Box<SuffixOperatorExpr>),
}

///
#[derive(Clone)]
pub struct BinaryOperatorExpr {
    pub operator: Operator,
    pub lhs: Expr,
    pub rhs: Expr,
}

///
#[derive(Clone)]
pub struct PrefixOperatorExpr {
    pub operator: Operator,
    pub rhs: Expr,
}

///
#[derive(Clone)]
pub struct SuffixOperatorExpr {
    pub operator: Operator,
    pub lhs: Expr,
}

///
#[derive(Clone)]
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
#[derive(Clone)]
pub struct RefExpr {
    pub referent: Expr,
}

///
#[derive(Clone)]
pub struct SelectExpr {
}

///
#[derive(Clone)]
pub struct TupleExpr {
}

///
#[derive(Clone)]
pub struct VoidExpr {
}

///
#[derive(Clone)]
pub enum Pattern {
    List(Box<ListPattern>),
    Tuple(Box<TuplePattern>),
    Variable(Box<VariablePattern>),
}

///
pub type Patterns = Vec<Pattern>;

///
#[derive(Clone)]
pub enum ListPattern {
    Cons(ConsListPattern),
    Item(ElementListPattern),
}

///
#[derive(Clone)]
pub struct ConsListPattern {
    pub head: Pattern,
    pub tail: Pattern,
}

///
#[derive(Clone)]
pub struct ElementListPattern {
    pub elements: Patterns,
}

///
#[derive(Clone)]
pub struct TuplePattern {
    pub fields: Patterns,
}

///
pub type VariablePattern = Identifier;


