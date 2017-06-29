//! # NoIR
//!
//! The normalised intermediate representation ([NoIR](libarvo.noir)) is the normalised form
//! of the AIR. NoIR is produced by normalising an AIR that has been through
//! all necessary compilation passes, and so by the time a program has reached
//! this stage of compilation, it is valid. Optimisation passes, and code
//! generation passes, are done using NoIR.

use std::collections::HashMap;

use super::identifier::{Identifier, Identify, Symbol, Symbolise};

#[macro_use]
pub mod macros;
pub mod context;
pub mod prelude;
pub mod runtime;

/// The `Typedef` trait is implemented by nodes that can express some `Type`.
pub trait Typedef {
    fn typedef(&self) -> Type;
}

///
#[derive(Clone)]
pub struct AssignExpr {
    identifier: Identifier,
    lhs: RefExpr,
    rhs: Expr,
}

impl AssignExpr {
    pub fn new<LHS, RHS>(identifier: Identifier, lhs: LHS, rhs: RHS) -> AssignExpr
        where LHS: Into<RefExpr>,
              RHS: Into<Expr>
    {
        AssignExpr {
            identifier: identifier,
            lhs: lhs.into(),
            rhs: rhs.into(),
        }
    }

    pub fn lhs(&self) -> &RefExpr {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expr {
        &self.rhs
    }
}

///
#[derive(Clone)]
pub struct BlockExpr {
    identifier: Identifier,
    body: Exprs,
    ret: Expr,
    functions: FunctionTable,
    types: TypeTable,
    variables: VariableTable,
}

impl BlockExpr {
    pub fn new<Body, Return, Fns, Tys, Vars>(identifier: Identifier,
                                             body: Body,
                                             ret: Return,
                                             functions: Fns,
                                             types: Tys,
                                             variables: Vars)
                                             -> BlockExpr
        where Body: Into<Exprs>,
              Return: Into<Expr>,
              Fns: Into<FunctionTable>,
              Tys: Into<TypeTable>,
              Vars: Into<VariableTable>
    {
        BlockExpr {
            identifier: identifier,
            body: body.into(),
            ret: ret.into(),
            functions: functions.into(),
            types: types.into(),
            variables: variables.into(),
        }
    }

    pub fn body(&self) -> &Exprs {
        &self.body
    }

    pub fn ret(&self) -> &Expr {
        &self.ret
    }

    pub fn functions(&self) -> &FunctionTable {
        &self.functions
    }

    pub fn types(&self) -> &TypeTable {
        &self.types
    }

    pub fn variables(&self) -> &VariableTable {
        &self.variables
    }
}

impl Identify for BlockExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for BlockExpr {
    fn typedef(&self) -> Type {
        self.ret.typedef()
    }
}

///
#[derive(Clone)]
pub struct CallExpr {
    identifier: Identifier,
    target: Expr,
    arguments: Exprs,
}

impl CallExpr {
    pub fn new<Target, Arguments>(identifier: Identifier,
                                  target: Target,
                                  arguments: Arguments)
                                  -> CallExpr
        where Target: Into<Expr>,
              Arguments: Into<Exprs>
    {
        CallExpr {
            identifier: identifier,
            target: target.into(),
            arguments: arguments.into(),
        }
    }

    pub fn target(&self) -> &Expr {
        &self.target
    }

    pub fn arguments(&self) -> &Exprs {
        &self.arguments
    }
}

impl Identify for CallExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for CallExpr {
    fn typedef(&self) -> Type {
        match self.target().typedef() {
            Type::Lambda(lambda_type) => lambda_type.ret().clone(),
            _ => unreachable!(),
        }
    }
}

///
#[derive(Clone)]
pub struct DefExpr {
    identifier: Identifier,
    variable: Variable,
    definition: Box<AssignExpr>,
}

impl DefExpr {
    pub fn new<Var, Definition>(identifier: Identifier,
                                variable: Var,
                                definition: Definition)
                                -> DefExpr
        where Var: Into<Variable>,
              Definition: Into<Box<AssignExpr>>
    {
        DefExpr {
            identifier: identifier,
            variable: variable.into(),
            definition: definition.into(),
        }
    }

    pub fn variable(&self) -> &Variable {
        &self.variable
    }

    pub fn definition(&self) -> &Box<AssignExpr> {
        &self.definition
    }
}

impl Identify for DefExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for DefExpr {
    fn typedef(&self) -> Type {
        Type::from(PrimitiveType::Void)
    }
}

///
#[derive(Clone)]
pub struct DerefExpr {
    identifier: Identifier,
    inner: Expr,
}

impl DerefExpr {
    pub fn new<Inner>(identifier: Identifier, inner: Inner) -> DerefExpr
        where Inner: Into<Expr>
    {
        DerefExpr {
            identifier: identifier,
            inner: inner.into(),
        }
    }

    pub fn inner(&self) -> &Expr {
        &self.inner
    }
}

impl Identify for DerefExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for DerefExpr {
    fn typedef(&self) -> Type {
        match self.inner.typedef() {
            Type::Ptr(ptr_type) => ptr_type.inner().clone(),
            Type::Ref(ref_type) => ref_type.inner().clone(),
        }
    }
}

///
#[derive(Clone)]
pub struct ForExpr {
    identifier: Identifier,
    variables: Variables,
    iterator: Expr,
    iteration: Box<BlockExpr>,
}

impl ForExpr {
    pub fn new<Vars, Iter, Iteration>(identifier: Identifier,
                                      variables: Vars,
                                      iterator: Iter,
                                      iteration: Iteration)
                                      -> ForExpr
        where Vars: Into<Variables>,
              Iter: Into<Expr>,
              Iteration: Into<Box<BlockExpr>>
    {
        ForExpr {
            identifier: identifier,
            variables: variables.into(),
            iterator: iterator.into(),
            iteration: iteration.into(),
        }
    }

    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    pub fn iterator(&self) -> &Expr {
        &self.iterator
    }

    pub fn iteration(&self) -> &Box<BlockExpr> {
        &self.iteration
    }
}

///
#[derive(Clone)]
pub struct Function {
    symbol: Symbol,
    formals: Variables,
    ret: Type,
    body: Box<Option<BlockExpr>>,
}

impl Function {
    pub fn new<Vars, Return, Body>(symbol: Symbol,
                                   formals: Vars,
                                   ret: Return,
                                   body: Body)
                                   -> Function
        where Vars: Into<Variables>,
              Return: Into<Type>,
              Body: Into<Box<Option<BlockExpr>>>
    {
        Function {
            symbol: symbol,
            formals: formals.into(),
            ret: ret.into(),
            body: body.into(),
        }
    }

    pub fn formals(&self) -> &Variables {
        &self.formals
    }

    pub fn ret(&self) -> &Type {
        &self.ret
    }

    pub fn body(&self) -> &Box<Option<BlockExpr>> {
        &self.body
    }
}

impl Typedef for Function {
    fn typedef(&self) -> Type {
        LambdaType::new(self.formals
                            .iter()
                            .map(|formal| formal.typedef())
                            .collect(),
                        self.ret.clone())
                .into()
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
pub type FunctionTable = HashMap<Identifier, Function>;

///
#[derive(Clone)]
pub struct IfExpr {
    identifier: Identifier,
    conditional: Expr,
    then_block: Box<BlockExpr>,
    else_block: Box<BlockExpr>,
}

impl IfExpr {
    pub fn new<Conditional, Then, Else>(identifier: Identifier,
                                        conditional: Conditional,
                                        then_block: Then,
                                        else_block: Else)
                                        -> IfExpr
        where Conditional: Into<Expr>,
              Then: Into<Box<BlockExpr>>,
              Else: Into<Box<BlockExpr>>
    {
        IfExpr {
            identifier: identifier,
            conditional: conditional.into(),
            then_block: then_block.into(),
            else_block: else_block.into(),
        }
    }

    pub fn conditional(&self) -> &Expr {
        &self.conditional
    }

    pub fn then_block(&self) -> &Box<BlockExpr> {
        &self.then_block
    }

    pub fn else_block(&self) -> &Box<BlockExpr> {
        &self.else_block
    }
}

impl Identify for IfExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for IfExpr {
    fn typedef(&self) -> Type {
        self.then_block.typedef()
    }
}

///
#[derive(Clone)]
pub struct ItemExpr {
    identifier: Identifier,
    item: Item,
}

impl ItemExpr {
    pub fn new(identifier: Identifier, item: Item) -> ItemExpr {
        ItemExpr {
            identifier: identifier,
            item: item,
        }
    }

    pub fn item(&self) -> &Item {
        &self.item
    }
}

impl Identify for ItemExpr {
    fn identify(&self) -> Identifier {
        self.item.identify()
    }
}

impl Symbolise for ItemExpr {
    fn symbolise(&self) -> Symbol {
        self.item.symbolise()
    }
}

impl Typedef for ItemExpr {
    fn typedef(&self) -> Type {
        self.item.typedef()
    }
}

///
#[derive(Clone)]
pub enum Item {
    Function(Function),
    Variable(Variable),
}

impl Identify for Item {
    fn identify(&self) -> Identifier {
        match *self {
            Item::Function(ref function) => function.identify(),
            Item::Variable(ref variable) => variable.identify(),
        }
    }
}

impl Symbolise for Item {
    fn symbolise(&self) -> Symbol {
        match *self {
            Item::Function(ref function) => function.symbolise(),
            Item::Variable(ref variable) => variable.symbolise(),
        }
    }
}

impl Typedef for Item {
    fn typedef(&self) -> Type {
        match *self {
            Item::Function(ref function) => function.typedef(),
            Item::Variable(ref variable) => variable.typedef(),
        }
    }
}

///
#[derive(Clone)]
pub struct LambdaType {
    formals: Types,
    ret: Type,
}

impl LambdaType {
    pub fn new<Formals, Return>(formals: Formals, ret: Return) -> LambdaType
        where Formals: Into<Types>,
              Return: Into<Type>
    {
        LambdaType {
            formals: formals.into(),
            ret: ret.into(),
        }
    }

    pub fn formals(&self) -> &Vec<Type> {
        &self.formals
    }

    pub fn ret(&self) -> &Type {
        &self.ret
    }
}

///
#[derive(Clone)]
pub struct LiteralExpr {
    identifier: Identifier,
    literal: Literal,
}

impl LiteralExpr {
    pub fn new<Lit>(identifier: Identifier, literal: Lit) -> LiteralExpr
        where Lit: Into<Literal>
    {
        LiteralExpr {
            identifier: identifier,
            literal: literal.into(),
        }
    }

    pub fn literal(&self) -> &Literal {
        &self.literal
    }
}

impl Identify for LiteralExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for LiteralExpr {
    fn typedef(&self) -> Type {
        match self.literal {
            Literal::Bool(..) => PrimitiveType::Bool.into(),
            Literal::Char(..) => PrimitiveType::Char.into(),
            Literal::F32(..) => PrimitiveType::F32.into(),
            Literal::F64(..) => PrimitiveType::F64.into(),
            Literal::I8(..) => PrimitiveType::I8.into(),
            Literal::I16(..) => PrimitiveType::I16.into(),
            Literal::I32(..) => PrimitiveType::I32.into(),
            Literal::I64(..) => PrimitiveType::I64.into(),
            Literal::Str(..) => PrimitiveType::Str.into(),
            Literal::U8(..) => PrimitiveType::U8.into(),
            Literal::U16(..) => PrimitiveType::U16.into(),
            Literal::U32(..) => PrimitiveType::U32.into(),
            Literal::U64(..) => PrimitiveType::U64.into(),
            Literal::USize(..) => PrimitiveType::USize.into(),
        }
    }
}

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
pub struct Module {
    symbol: Symbol,
    functions: FunctionTable,
    modules: ModuleTable,
    types: TypeTable,
}

impl Module {
    pub fn new<Fns, Mods, Tys>(symbol: Symbol, functions: Fns, modules: Mods, types: Tys) -> Module
        where Fns: Into<FunctionTable>,
              Mods: Into<ModuleTable>,
              Tys: Into<TypeTable>
    {
        Module {
            symbol: symbol,
            functions: functions.into(),
            modules: modules.into(),
            types: types.into(),
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.insert(function.identify(), function);
    }

    pub fn functions(&self) -> &FunctionTable {
        &self.functions
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.insert(module.identify(), module);
    }

    pub fn modules(&self) -> &ModuleTable {
        &self.modules
    }

    pub fn add_struct_type(&mut self, struct_type: StructType) {
        self.types
            .insert(struct_type.identify(), struct_type.into());
    }

    pub fn types(&self) -> &TypeTable {
        &self.types
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
pub type ModuleTable = HashMap<Identifier, Module>;

///
#[derive(Clone)]
pub struct ProcessExpr {
    identifier: Identifier,
    body: Box<BlockExpr>,
}

impl ProcessExpr {
    pub fn new<Body>(identifier: Identifier, body: Body) -> ProcessExpr
        where Body: Into<Box<BlockExpr>>
    {
        ProcessExpr {
            identifier: identifier,
            body: body.into(),
        }
    }

    pub fn body(&self) -> &Box<BlockExpr> {
        &self.body
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
    identifier: Identifier,
    process: ProcessExpr,
}

impl ProcessJoinExpr {
    pub fn new<Process>(identifier: Identifier, process: Process) -> ProcessJoinExpr
        where Process: Into<ProcessExpr>
    {
        ProcessJoinExpr {
            identifier: identifier,
            process: process.into(),
        }
    }

    pub fn process(&self) -> &ProcessExpr {
        &self.process
    }
}

///
#[derive(Clone)]
pub struct PtrType {
    inner: Type,
}

impl PtrType {
    pub fn new<Inner>(inner: Inner) -> PtrType
        where Inner: Into<Type>
    {
        PtrType { inner: inner.into() }
    }

    pub fn inner(&self) -> &Type {
        &self.inner
    }
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
pub struct RefExpr {
    identifier: Identifier,
    inner: Expr,
    mutable: bool,
}

impl RefExpr {
    pub fn new<Inner, Mutable>(identifier: Identifier, inner: Inner, mutable: Mutable) -> RefExpr
        where Inner: Into<Expr>,
              Mutable: Into<bool>
    {
        RefExpr {
            identifier: identifier,
            inner: inner.into(),
            mutable: mutable.into(),
        }
    }

    pub fn inner(&self) -> &Expr {
        &self.inner
    }

    pub fn mutable(&self) -> bool {
        self.mutable
    }
}

impl Identify for RefExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for RefExpr {
    fn typedef(&self) -> Type {
        Type::from(RefType::new(self.inner.typedef(), self.mutable))
    }
}

///
#[derive(Clone)]
pub struct RefType {
    inner: Type,
    mutable: bool,
}

impl RefType {
    pub fn new<Inner, Mutable>(inner: Inner, mutable: Mutable) -> RefType
        where Inner: Into<Type>,
              Mutable: Into<bool>
    {
        RefType {
            inner: inner.into(),
            mutable: mutable.into(),
        }
    }

    pub fn inner(&self) -> &Type {
        &self.inner
    }

    pub fn mutable(&self) -> bool {
        self.mutable
    }
}

///
#[derive(Clone)]
pub struct StructExpr {
    identifier: Identifier,
    elements: Exprs,
    ty: Box<StructType>,
}

impl StructExpr {
    pub fn new<Elements, Ty>(identifier: Identifier, elements: Elements, ty: Ty) -> StructType
        where Elements: Into<Exprs>,
              Ty: Into<Box<StructType>>
    {
        StructExpr {
            identifier: identifier,
            elements: elements.into(),
            ty: ty.into(),
        }
    }

    pub fn elements(&self) -> &Exprs {
        &self.elements
    }
}

impl Identify for StructExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for StructExpr {
    fn typedef(&self) -> Type {
        self.ty.clone()
    }
}

///
#[derive(Clone)]
pub struct StructAccessExpr {
    identifier: Identifier,
    target: Expr,
    element: Variable,
}

impl StructAccessExpr {
    pub fn new<Target, Element>(identifier: Identifier,
                                target: Target,
                                element: Element)
                                -> StructAccessExpr
        where Target: Into<Expr>,
              Element: Into<Variable>
    {
        StructAccessExpr {
            identifier: identifier,
            target: target.into(),
            element: element.into(),
        }
    }

    pub fn target(&self) -> &Expr {
        &self.target
    }

    pub fn element(&self) -> &Variable {
        &self.element
    }
}

impl Identify for StructAccessExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl Typedef for StructAccessExpr {
    fn typedef(&self) -> Type {
        self.element.typedef()
    }
}

///
#[derive(Clone)]
pub struct StructType {
    symbol: Symbol,
    elements: Variables,
}

impl StructType {
    pub fn new<Vars>(symbol: Symbol, elements: Vars) -> StructType
        where Vars: Into<Variables>
    {
        StructType {
            symbol: symbol,
            elements: elements.into(),
        }
    }

    pub fn elements(&self) -> &Variables {
        &self.elements
    }
}

impl Identify for StructType {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for StructType {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct Variable {
    symbol: Symbol,
    ty: Type,
}

impl Variable {
    pub fn new<Ty>(symbol: Symbol, ty: Ty) -> Variable
        where Ty: Into<Type>
    {
        Variable {
            symbol: symbol,
            ty: ty.into(),
        }
    }
}

impl Typedef for Variable {
    fn typedef(&self) -> Type {
        self.ty.clone()
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
pub type VariableTable = HashMap<Identifier, Variable>;

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
    Def(Box<DefExpr>),
    Deref(Box<DerefExpr>),
    For(Box<ForExpr>),
    If(Box<IfExpr>),
    Item(Box<ItemExpr>),
    Literal(Box<LiteralExpr>),
    Process(Box<ProcessExpr>),
    ProcessJoin(Box<ProcessJoinExpr>),
    Ref(Box<RefExpr>),
    Struct(Box<StructExpr>),
    StructAccess(Box<StructAccessExpr>),
    Void(Box<VoidExpr>),
}

impl Typedef for Expr {
    fn typedef(&self) -> Type {
        expr_proxy![*self => typedef()]
    }
}

impl From<AssignExpr> for Expr {
    fn from(expr: AssignExpr) -> Expr {
        Expr::Assign(expr.into())
    }
}

impl From<BlockExpr> for Expr {
    fn from(expr: BlockExpr) -> Expr {
        Expr::Block(expr.into())
    }
}

impl From<CallExpr> for Expr {
    fn from(expr: CallExpr) -> Expr {
        Expr::Call(expr.into())
    }
}

impl From<DefExpr> for Expr {
    fn from(expr: DefExpr) -> Expr {
        Expr::Def(expr.into())
    }
}

impl From<DerefExpr> for Expr {
    fn from(expr: DerefExpr) -> Expr {
        Expr::Deref(expr.into())
    }
}

impl From<ForExpr> for Expr {
    fn from(expr: ForExpr) -> Expr {
        Expr::For(expr.into())
    }
}

impl From<IfExpr> for Expr {
    fn from(expr: IfExpr) -> Expr {
        Expr::If(expr.into())
    }
}

impl From<ItemExpr> for Expr {
    fn from(expr: ItemExpr) -> Expr {
        Expr::Item(expr.into())
    }
}

impl From<LiteralExpr> for Expr {
    fn from(expr: LiteralExpr) -> Expr {
        Expr::Literal(expr.into())
    }
}

impl From<ProcessExpr> for Expr {
    fn from(expr: ProcessExpr) -> Expr {
        Expr::Process(expr.into())
    }
}

impl From<ProcessJoinExpr> for Expr {
    fn from(expr: ProcessJoinExpr) -> Expr {
        Expr::ProcessJoin(expr.into())
    }
}

impl From<RefExpr> for Expr {
    fn from(expr: RefExpr) -> Expr {
        Expr::Ref(expr.into())
    }
}

impl From<StructExpr> for Expr {
    fn from(expr: StructExpr) -> Expr {
        Expr::Struct(expr.into())
    }
}

impl From<StructAccessExpr> for Expr {
    fn from(expr: StructAccessExpr) -> Expr {
        Expr::StructAccess(expr.into())
    }
}

impl From<RefExpr> for Expr {
    fn from(expr: RefExpr) -> Expr {
        Expr::Ref(expr.into())
    }
}

impl From<VoidExpr> for Expr {
    fn from(expr: VoidExpr) -> Expr {
        Expr::Void(expr.into())
    }
}

///
pub type Exprs = Vec<Expr>;

///
#[derive(Clone)]
pub enum Type {
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

impl From<RefType> for Type {
    fn from(ty: RefType) -> Type {
        Type::Ref(ty.into())
    }
}

impl From<StructType> for Type {
    fn from(ty: StructType) -> Type {
        Type::Struct(ty.into())
    }
}

///
pub type Types = Vec<Type>;
pub type TypeTable = HashMap<Identifier, Type>;