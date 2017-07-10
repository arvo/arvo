//! # NoIR
//!
//! The normalised intermediate representation ([NoIR](libarvo.noir)) is the normalised form
//! of the AIR. NoIR is produced by normalising an AIR that has been through
//! all necessary compilation passes, and so by the time a program has reached
//! this stage of compilation, it is valid. Optimisation passes, and code
//! generation passes, are done using NoIR.

use super::identifier::{Identifier, Id, Identify, Name, Symbol, Symbolise};

use std::collections::HashMap;

#[macro_use]
pub mod macros;
pub mod context;
pub mod prelude;
pub mod runtime;

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
    pub ret: Expr,
    pub epilogue: Exprs,
    pub function_table: FunctionTable,
    pub module_table: ModuleTable,
    pub type_table: TypeTable,
}

impl BlockExpr {
    pub fn new<P, E, B, R>(prelude: Vec<P>,
                           epilogue: Vec<E>,
                           body: Vec<B>,
                           ret: R,
                           function_table: FunctionTable,
                           module_table: ModuleTable,
                           type_table: TypeTable)
                           -> BlockExpr
        where P: Into<Expr>,
              E: Into<Expr>,
              B: Into<Expr>,
              R: Into<Expr>
    {
        BlockExpr::new_with_id(Identifier::id(),
                               prelude,
                               epilogue,
                               body,
                               ret,
                               function_table,
                               module_table,
                               type_table)
    }

    pub fn new_with_id<P, E, B, R>(identifier: Identifier,
                                   prelude: Vec<P>,
                                   epilogue: Vec<E>,
                                   body: Vec<B>,
                                   ret: R,
                                   function_table: FunctionTable,
                                   module_table: ModuleTable,
                                   type_table: TypeTable)
                                   -> BlockExpr
        where P: Into<Expr>,
              E: Into<Expr>,
              B: Into<Expr>,
              R: Into<Expr>
    {
        BlockExpr {
            identifier: identifier,
            prelude: prelude.into_iter().map(|expr| expr.into()).collect(),
            epilogue: epilogue.into_iter().map(|expr| expr.into()).collect(),
            body: body.into_iter().map(|expr| expr.into()).collect(),
            ret: ret.into(),
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
    pub fn new<T, A>(target: T, arguments: Vec<A>) -> CallExpr
        where T: Into<Expr>,
              A: Into<Expr>
    {
        CallExpr::new_with_id(Identifier::id(), target, arguments)
    }

    pub fn new_with_id<T, A>(identifier: Identifier, target: T, arguments: Vec<A>) -> CallExpr
        where T: Into<Expr>,
              A: Into<Expr>
    {
        CallExpr {
            identifier: identifier,
            target: target.into(),
            arguments: arguments
                .into_iter()
                .map(|argument| argument.into())
                .collect(),
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
    pub mangled_name: String,
}

impl Function {
    pub fn new<S, F, R, B>(symbol: S, formals: Vec<F>, ret: R, body: B) -> Function
        where S: Into<Symbol>,
              F: Into<Variable>,
              R: Into<Type>,
              B: Into<Option<BlockExpr>>
    {
        let symbol = symbol.into();
        Function {
            symbol: symbol.clone(),
            formals: formals
                .into_iter()
                .map(|formal| formal.into())
                .collect(),
            ret: ret.into(),
            body: body.into(),
            mangled_name: symbol.name(),
        }
    }

    pub fn is_extern(&self) -> bool {
        if let Some(..) = self.body {
            false
        } else {
            true
        }
    }

    pub fn is_void(&self) -> bool {
        if let Type::Primitive(ref ty) = self.ret {
            if let PrimitiveType::Void = *ty.as_ref() {
                return true;
            }
        }
        false
    }

    pub fn lambda_type(&self) -> LambdaType {
        LambdaType::new(self.formals
                            .iter()
                            .map(|formal| formal.ty.clone())
                            .collect(),
                        self.ret.clone())
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
    pub identifier: Identifier,
    pub item: Item,
}

impl ItemExpr {
    pub fn new<I: Into<Item>>(item: I) -> ItemExpr {
        ItemExpr::new_with_id(Identifier::id(), item)
    }

    pub fn new_with_id<I: Into<Item>>(identifier: Identifier, item: I) -> ItemExpr {
        ItemExpr {
            identifier: identifier,
            item: item.into(),
        }
    }
}

impl Symbolise for ItemExpr {
    fn symbolise(&self) -> Symbol {
        self.item.symbolise()
    }
}

///
#[derive(Clone)]
pub struct LambdaType {
    pub formals: Types,
    pub ret: Type,
}

impl LambdaType {
    pub fn new<F, R>(formals: Vec<F>, ret: R) -> LambdaType
        where F: Into<Type>,
              R: Into<Type>
    {
        LambdaType {
            formals: formals
                .into_iter()
                .map(|formal| formal.into())
                .collect(),
            ret: ret.into(),
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

impl LiteralExpr {
    pub fn new<L: Into<Literal>>(literal: L) -> LiteralExpr {
        LiteralExpr::new_with_id(Identifier::id(), literal)
    }

    pub fn new_with_id<L: Into<Literal>>(identifier: Identifier, literal: L) -> LiteralExpr {
        LiteralExpr {
            identifier: identifier,
            literal: literal.into(),
        }
    }
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

impl ProcessExpr {
    pub fn new<B: Into<Box<BlockExpr>>>(body: B) -> ProcessExpr {
        ProcessExpr::new_with_id(Identifier::id(), body)
    }

    pub fn new_with_id<B: Into<Box<BlockExpr>>>(identifier: Identifier, body: B) -> ProcessExpr {
        ProcessExpr {
            identifier: identifier,
            body: body.into(),
        }
    }

    pub fn function(&self) -> Function {
        Function::new(
            Symbol::new(format!("__libruntime__process_{}", self.id())),
            Variables::new(),
            PrimitiveType::Void,
            self.body.as_ref().clone(),
        )
    }
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

impl ProcessJoinExpr {
    pub fn new<P>(process_expr: P) -> ProcessJoinExpr
        where P: Into<Box<ProcessExpr>>
    {
        ProcessJoinExpr::new_with_id(Identifier::id(), process_expr)
    }

    pub fn new_with_id<P>(identifier: Identifier, process_expr: P) -> ProcessJoinExpr
        where P: Into<Box<ProcessExpr>>
    {
        ProcessJoinExpr {
            identifier: identifier,
            process_expr: process_expr.into(),
        }
    }
}

impl Identify for ProcessJoinExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct PtrType {
    pub inner: Type,
}

impl PtrType {
    fn new<I>(inner: I) -> PtrType
        where I: Into<Type>
    {
        PtrType { inner: inner.into() }
    }
}

///
#[derive(Clone)]
pub struct RefType {
    pub inner: Type,
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
    pub fn new<S, T>(symbol: S, ty: T) -> Variable
        where S: Into<Symbol>,
              T: Into<Type>
    {
        Variable {
            symbol: symbol.into(),
            ty: ty.into(),
        }
    }
}

impl Identify for Variable {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
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
    pub fn new() -> VoidExpr {
        VoidExpr::new_with_id(Identifier::id())
    }

    pub fn new_with_id(identifier: Identifier) -> VoidExpr {
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

impl From<ItemExpr> for Expr {
    fn from(item_expr: ItemExpr) -> Expr {
        Expr::Item(item_expr.into())
    }
}

impl From<LiteralExpr> for Expr {
    fn from(literal_expr: LiteralExpr) -> Expr {
        Expr::Literal(literal_expr.into())
    }
}

impl From<ProcessExpr> for Expr {
    fn from(process_expr: ProcessExpr) -> Expr {
        Expr::Process(process_expr.into())
    }
}

impl From<ProcessJoinExpr> for Expr {
    fn from(process_join_expr: ProcessJoinExpr) -> Expr {
        Expr::ProcessJoin(process_join_expr.into())
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

impl Symbolise for Item {
    fn symbolise(&self) -> Symbol {
        match *self {
            Item::Function(ref function) => function.symbolise(),
            _ => unimplemented!(),
        }
    }
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
    Char(char),
    F32(f32),
    F64(f64),
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