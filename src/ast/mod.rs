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
    pub body: Option<RHSExpr>,
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
    EnumRecord(EnumRecordVariant),
    EnumNamedRecord(EnumNamedRecordVariant),
}

///
pub type EnumVariants = Vec<EnumVariant>;

///
#[derive(Clone)]
pub struct EnumRecordVariant {
    pub identifier: Identifier,
    pub fields: Types,
}

///
#[derive(Clone)]
pub struct EnumNamedRecordVariant {
    pub identifier: Identifier,
    pub fields: EnumNamedRecordVariantFields,
}

///
#[derive(Clone)]
pub struct EnumNamedRecordVariantField {
    pub identifier: Identifier,
    pub ty: Type,
}

///
pub type EnumNamedRecordVariantFields = Vec<EnumNamedRecordVariantField>;

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
    Channel(ChannelType),
    List(ListType),
    Optional(OptionalType),
    Tuple(TupleType),
    Unresolved(UnresolvedType),
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
pub enum RHSExpr {
    Assign(AssignExpr),
    Block(BlockExpr),
    Call(CallExpr),
    Channel(ChannelExpr),
    Deref(DerefExpr),
    For(ForExpr),
    If(IfExpr),
    Item(ItemExpr),
    List(ListExpr),
    Literal(LiteralExpr),
    Operator(OperatorExpr),
    Ref(RefExpr),
    Select(SelectExpr),
    Tuple(TupleExpr),
    Void(VoidExpr),
}