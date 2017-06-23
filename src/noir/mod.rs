//! Normalized Intermediate Representation

use std::collections::HashMap;

use super::bair::{Object, PrimitiveType};
use super::identifier::{Identifier, Identify, Id, Symbol, Symbolise};

pub mod prelude;
pub mod runtime;
pub mod visitor;

/// The `Typedef` trait is implemented by nodes that can express some `Type`.
pub trait Typedef {
    fn typedef(&self) -> Type;
}

///
#[derive(Clone)]
pub struct BlockExpr(Object<RawBlockExpr>);

impl BlockExpr {
    pub fn new(identifier: Identifier, ret: Expr, body: Exprs, epilogue: Exprs) -> BlockExpr {
        BlockExpr(object!(RawBlockExpr::new(identifier, ret, body, epilogue)))
    }

    pub fn ret(&self) -> Expr {
        object_proxy![self.0 => ret().clone()]
    }

    pub fn for_body_exprs<T, F: FnMut(&mut Expr) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => body_mut().iter_mut() loop expr {
            f(expr);
        }]
    }

    pub fn num_body_exprs(&self) -> usize {
        object_proxy![self.0 => body().len()]
    }

    pub fn for_epilogue_exprs<T, F: FnMut(&mut Expr) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => epilogue_mut().iter_mut() loop expr {
            f(expr);
        }]
    }

    pub fn num_epilogue_exprs(&self) -> usize {
        object_proxy![self.0 => epilogue().len()]
    }
}

impl Identify for BlockExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

#[derive(Clone)]
struct RawBlockExpr {
    identifier: Identifier,
    ret: Expr,
    body: Exprs,
    epilogue: Exprs,
}

impl RawBlockExpr {
    fn new(identifier: Identifier, ret: Expr, body: Exprs, epilogue: Exprs) -> RawBlockExpr {
        RawBlockExpr {
            identifier: identifier,
            ret: ret,
            body: body,
            epilogue: epilogue,
        }
    }

    fn ret(&self) -> &Expr {
        &self.ret
    }

    fn body(&self) -> &Exprs {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Exprs {
        &mut self.body
    }

    fn epilogue(&self) -> &Exprs {
        &self.epilogue
    }

    fn epilogue_mut(&mut self) -> &mut Exprs {
        &mut self.epilogue
    }
}

impl Identify for RawBlockExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct CallExpr(Object<RawCallExpr>);

impl CallExpr {
    pub fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> CallExpr {
        CallExpr(object!(RawCallExpr::new(identifier, target, arguments)))
    }

    pub fn target(&self) -> Expr {
        object_proxy![self.0 => target().clone()]
    }

    pub fn for_arguments<T, F: FnMut(&mut Expr) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => arguments_mut().iter_mut() loop argument {
            f(argument);
        }]
    }

    pub fn map_arguments<T, F: FnMut(&mut Expr) -> T>(&self, f: F) -> Vec<T> {
        object_proxy_mut![self.0 => arguments_mut().iter_mut().map(f).collect()]
    }

    pub fn num_arguments(&self) -> usize {
        object_proxy![self.0 => arguments().len()]
    }
}

impl Identify for CallExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
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

    fn target(&self) -> &Expr {
        &self.target
    }

    fn arguments(&self) -> &Exprs {
        &self.arguments
    }

    fn arguments_mut(&mut self) -> &mut Exprs {
        &mut self.arguments
    }
}

impl Identify for RawCallExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
pub struct Context {
    prelude: prelude::Prelude,
    runtime: runtime::Runtime,
}

impl Context {
    pub fn new() -> Context {
        Context {
            prelude: prelude::Prelude::new(),
            runtime: runtime::Runtime::new(),
        }
    }

    pub fn prelude(&self) -> &prelude::Prelude {
        &self.prelude
    }

    pub fn runtime(&self) -> &runtime::Runtime {
        &self.runtime
    }
}

///
#[derive(Clone)]
pub struct DefExpr(Object<RawDefExpr>);

impl DefExpr {
    pub fn new(identifier: Identifier, variable: Variable, definition: Expr) -> DefExpr {
        DefExpr(object!(RawDefExpr::new(identifier, variable, definition)))
    }
}

#[derive(Clone)]
pub struct RawDefExpr {
    identifier: Identifier,
    variable: Variable,
    definition: Expr,
}

impl RawDefExpr {
    fn new(identifier: Identifier, variable: Variable, definition: Expr) -> RawDefExpr {
        RawDefExpr {
            identifier: identifier,
            variable: variable,
            definition: definition,
        }
    }
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
    Item(ItemExpr),
    Literal(LiteralExpr),
    Process(ProcessExpr),
    ProcessJoin(ProcessJoinExpr),
    Struct(StructExpr),
    StructElement(StructElementExpr),
    Void(VoidExpr),
}
pub type Exprs = Vec<Expr>;

impl From<BlockExpr> for Expr {
    fn from(block_expr: BlockExpr) -> Expr {
        Expr::Block(block_expr)
    }
}

impl From<CallExpr> for Expr {
    fn from(call_expr: CallExpr) -> Expr {
        Expr::Call(call_expr)
    }
}

impl From<LiteralExpr> for Expr {
    fn from(literal_expr: LiteralExpr) -> Expr {
        Expr::Literal(literal_expr)
    }
}

impl From<ItemExpr> for Expr {
    fn from(item_expr: ItemExpr) -> Expr {
        Expr::Item(item_expr)
    }
}

impl From<ProcessExpr> for Expr {
    fn from(process_expr: ProcessExpr) -> Expr {
        Expr::Process(process_expr)
    }
}

impl From<ProcessJoinExpr> for Expr {
    fn from(process_join_expr: ProcessJoinExpr) -> Expr {
        Expr::ProcessJoin(process_join_expr)
    }
}

impl From<VoidExpr> for Expr {
    fn from(void_expr: VoidExpr) -> Expr {
        Expr::Void(void_expr)
    }
}

///
#[derive(Clone)]
pub struct Function(Object<RawFunction>);
pub type Functions = HashMap<Identifier, Function>;

impl Function {
    pub fn new(symbol: Symbol,
               formals: Variables,
               block: Option<BlockExpr>,
               ty: LambdaType)
               -> Function {
        Function(object!(RawFunction::new(symbol, formals, block, ty)))
    }

    pub fn for_formals<T, F: FnMut(&mut Variable) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => formals_mut().iter_mut() loop formal {
            f(formal);
        }]
    }

    pub fn map_formals<T, F: FnMut(&mut Variable) -> T>(&self, f: F) -> Vec<T> {
        object_proxy_mut![self.0 => formals_mut().iter_mut().map(f).collect()]
    }

    pub fn num_formals(&self) -> usize {
        object_proxy![self.0 => formals().len()]
    }

    pub fn block(&self) -> Option<BlockExpr> {
        object_proxy![self.0 => block().clone()]
    }

    pub fn ty(&self) -> LambdaType {
        object_proxy![self.0 => ty().clone()]
    }
}

impl Typedef for Function {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

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

#[derive(Clone)]
struct RawFunction {
    symbol: Symbol,
    formals: Variables,
    block: Option<BlockExpr>,
    ty: LambdaType,
}

impl RawFunction {
    fn new(symbol: Symbol,
           formals: Variables,
           block: Option<BlockExpr>,
           ty: LambdaType)
           -> RawFunction {
        RawFunction {
            symbol: symbol,
            formals: formals,
            block: block,
            ty: ty,
        }
    }

    fn formals(&self) -> &Variables {
        &self.formals
    }

    fn formals_mut(&mut self) -> &mut Variables {
        &mut self.formals
    }

    fn block(&self) -> &Option<BlockExpr> {
        &self.block
    }

    fn ty(&self) -> &LambdaType {
        &self.ty
    }
}

impl Typedef for RawFunction {
    fn typedef(&self) -> Type {
        Type::Lambda(self.ty.clone())
    }
}

impl Identify for RawFunction {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for RawFunction {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub enum ItemExpr {
    Function(Function),
    Variable(Variable),
}

///
#[derive(Clone)]
pub struct LambdaType(Object<RawLambdaType>);

impl LambdaType {
    pub fn new(formals: Vec<Type>, ty: Type) -> LambdaType {
        LambdaType(object!(RawLambdaType::new(formals, ty)))
    }

    pub fn for_formals<T, F: FnMut(&mut Type) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => formals_mut().iter_mut() loop formal {
            f(formal);
        }]
    }

    pub fn map_formals<T, F: FnMut(&mut Type) -> T>(&self, f: F) -> Vec<T> {
        object_proxy_mut![self.0 => formals_mut().iter_mut().map(f).collect()]
    }

    pub fn num_formals(&self) -> usize {
        object_proxy![self.0 => formals().len()]
    }
}

impl Typedef for LambdaType {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
pub struct RawLambdaType {
    formals: Vec<Type>,
    ty: Type,
}

impl RawLambdaType {
    fn new(formals: Vec<Type>, ty: Type) -> RawLambdaType {
        RawLambdaType {
            formals: formals,
            ty: ty,
        }
    }

    fn formals(&self) -> &Vec<Type> {
        &self.formals
    }

    fn formals_mut(&mut self) -> &mut Vec<Type> {
        &mut self.formals
    }
}

impl Typedef for RawLambdaType {
    fn typedef(&self) -> Type {
        self.ty.clone()
    }
}

///
#[derive(Clone)]
pub enum LiteralExpr {
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
pub struct Module(Object<RawModule>);

impl Module {
    pub fn new(symbol: Symbol, functions: Functions) -> Module {
        Module(object!(RawModule::new(symbol, functions)))
    }

    pub fn add_function(&mut self, function: Function) {
        object_proxy_mut![self.0 => add_function(function)]
    }

    pub fn for_functions<T, F: FnMut((&Identifier, &mut Function)) -> T>(&self, mut f: F) {
        object_proxy_mut![self.0 => functions_mut().iter_mut() loop function {
            f(function);
        }]
    }

    pub fn map_functions<T, F: FnMut((&Identifier, &mut Function)) -> T>(&self, f: F) -> Vec<T> {
        object_proxy_mut![self.0 => functions_mut().iter_mut().map(f).collect()]
    }

    pub fn num_functions(&self) -> usize {
        object_proxy![self.0 => functions().len()]
    }
}

impl Identify for Module {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for Module {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

#[derive(Clone)]
pub struct RawModule {
    // identification
    symbol: Symbol,

    // functions defined in this module
    functions: Functions,
}

impl RawModule {
    fn new(symbol: Symbol, functions: Functions) -> RawModule {
        RawModule {
            symbol: symbol,
            functions: functions,
        }
    }

    fn add_function(&mut self, function: Function) {
        self.functions
            .insert(function.identify().clone(), function);
    }

    fn functions(&self) -> &Functions {
        &self.functions
    }

    fn functions_mut(&mut self) -> &mut Functions {
        &mut self.functions
    }
}

impl Identify for RawModule {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for RawModule {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct ProcessExpr(Object<RawProcessExpr>);

impl ProcessExpr {
    pub fn new(identifier: Identifier,
               prelude_exprs: Exprs,
               prelude_variables: Variables,
               body: Exprs)
               -> ProcessExpr {
        ProcessExpr(object!(RawProcessExpr::new(identifier,
                                                prelude_exprs,
                                                prelude_variables,
                                                body)))
    }

    pub fn as_function(&self) -> Function {
        let process = self.0.read().unwrap();
        let process = process.borrow();
        let block_expr = BlockExpr::new(Identifier::id(),
                                        Expr::from(VoidExpr::new(Identifier::id())),
                                        process.body().clone(),
                                        Exprs::new());

        Function::new(Symbol::new(format!("__libruntime__process_{}", process.id())),
                      Variables::new(),
                      Some(block_expr),
                      LambdaType::new(Types::new(), Type::Primitive(PrimitiveType::Void)))
    }
}

impl Identify for ProcessExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

#[derive(Clone)]
struct RawProcessExpr {
    identifier: Identifier,
    prelude_exprs: Exprs,
    prelude_variables: Variables,
    body: Exprs,
}

impl RawProcessExpr {
    fn new(identifier: Identifier,
           prelude_exprs: Exprs,
           prelude_variables: Variables,
           body: Exprs)
           -> RawProcessExpr {
        RawProcessExpr {
            identifier: identifier,
            prelude_exprs: prelude_exprs,
            prelude_variables: prelude_variables,
            body: body,
        }
    }

    fn body(&self) -> &Exprs {
        &self.body
    }
}

impl Identify for RawProcessExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct ProcessJoinExpr(Object<RawProcessJoinExpr>);

impl ProcessJoinExpr {
    pub fn new(identifier: Identifier, process: ProcessExpr) -> ProcessJoinExpr {
        ProcessJoinExpr(object!(RawProcessJoinExpr::new(identifier, process)))
    }

    pub fn process(&self) -> ProcessExpr {
        object_proxy![self.0 => process().clone()]
    }
}

#[derive(Clone)]
struct RawProcessJoinExpr {
    identifier: Identifier,
    process: ProcessExpr,
}

impl RawProcessJoinExpr {
    fn new(identifier: Identifier, process: ProcessExpr) -> RawProcessJoinExpr {
        RawProcessJoinExpr {
            identifier: identifier,
            process: process,
        }
    }

    fn process(&self) -> &ProcessExpr {
        &self.process
    }
}

///
#[derive(Clone)]
pub struct PtrType(Object<RawPtrType>);

impl PtrType {
    pub fn new(ty: Type) -> PtrType {
        PtrType(object!(RawPtrType::new(ty)))
    }

    pub fn ty(&self) -> Type {
        object_proxy![self.0 => ty().clone()]
    }
}

#[derive(Clone)]
struct RawPtrType {
    ty: Type,
}

impl RawPtrType {
    fn new(ty: Type) -> RawPtrType {
        RawPtrType { ty: ty }
    }

    fn ty(&self) -> &Type {
        &self.ty
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

///
#[derive(Clone)]
pub struct StructElementExpr(Object<RawStructElementExpr>);

impl StructElementExpr {
    pub fn new(identifier: Identifier, target: Expr, index: u64) -> StructElementExpr {
        StructElementExpr(object!(RawStructElementExpr::new(identifier, target, index)))
    }
}

#[derive(Clone)]
struct RawStructElementExpr {
    identifier: Identifier,
    target: Expr,
    index: u64,
}

impl RawStructElementExpr {
    fn new(identifier: Identifier, target: Expr, index: u64) -> RawStructElementExpr {
        RawStructElementExpr {
            identifier: identifier,
            target: target,
            index: index,
        }
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
    Ptr(PtrType),
    Struct(StructType),
}
pub type Types = Vec<Type>;

///
#[derive(Clone)]
pub struct Variable(Object<RawVariable>);
pub type Variables = Vec<Variable>;

impl Variable {
    pub fn new(symbol: Symbol, ty: Type) -> Variable {
        Variable(object!(RawVariable::new(symbol, ty)))
    }
}

impl Typedef for Variable {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

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

#[derive(Clone)]
struct RawVariable {
    // identification
    symbol: Symbol,

    // type
    ty: Type,
}

impl RawVariable {
    fn new(symbol: Symbol, ty: Type) -> RawVariable {
        RawVariable {
            symbol: symbol,
            ty: ty,
        }
    }
}

impl Typedef for RawVariable {
    fn typedef(&self) -> Type {
        self.ty.clone()
    }
}

impl Identify for RawVariable {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for RawVariable {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct VoidExpr(Object<RawVoidExpr>);

impl VoidExpr {
    pub fn new(identifier: Identifier) -> VoidExpr {
        VoidExpr(object!(RawVoidExpr::new(identifier)))
    }
}

#[derive(Clone)]
struct RawVoidExpr {
    identifier: Identifier,
}

impl RawVoidExpr {
    fn new(identifier: Identifier) -> RawVoidExpr {
        RawVoidExpr { identifier: identifier }
    }
}