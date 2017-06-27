//! # Normalised Intermediate Representation
//!
//! The normalised intermediate representation (NoIR) is the normalised form
//! of the AIR. NoIR is produced by normalising an AIR that has been through
//! all necessary compilation passes, and so by the time a program has reached
//! this stage of compilation, it is valid. Optimisation passes, and code
//! generation passes, are done using NoIR.

use std::collections::HashMap;

use super::bair::{Object, PrimitiveType};
use super::identifier::{Identifier, Identify, Id, Symbol, Symbolise};

#[macro_use]
pub mod macros;
pub mod prelude;
pub mod runtime;

/// The `Typedef` trait is implemented by nodes that can express some `Type`.
pub trait Typedef {
    fn typedef(&self) -> Type;
}

///
#[derive(Clone)]
pub struct BlockExpr(Object<_BlockExpr>);

impl BlockExpr {
    pub fn new(identifier: Identifier, ret: Expr, body: Exprs, epilogue: Exprs) -> BlockExpr {
        BlockExpr(object!(_BlockExpr::new(identifier, ret, body, epilogue)))
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
struct _BlockExpr {
    identifier: Identifier,
    ret: Expr,
    body: Exprs,
    epilogue: Exprs,
}

impl _BlockExpr {
    fn new(identifier: Identifier, ret: Expr, body: Exprs, epilogue: Exprs) -> _BlockExpr {
        _BlockExpr {
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

impl Identify for _BlockExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct CallExpr(Object<_CallExpr>);

impl CallExpr {
    pub fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> CallExpr {
        CallExpr(object!(_CallExpr::new(identifier, target, arguments)))
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
struct _CallExpr {
    identifier: Identifier,
    target: Expr,
    arguments: Exprs,
}

impl _CallExpr {
    fn new(identifier: Identifier, target: Expr, arguments: Exprs) -> _CallExpr {
        _CallExpr {
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

impl Identify for _CallExpr {
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
pub struct DefExpr(Object<_DefExpr>);

impl DefExpr {
    pub fn new(identifier: Identifier, variable: Variable, definition: Expr) -> DefExpr {
        DefExpr(object!(_DefExpr::new(identifier, variable, definition)))
    }
}

#[derive(Clone)]
struct _DefExpr {
    identifier: Identifier,
    variable: Variable,
    definition: Expr,
}

impl _DefExpr {
    fn new(identifier: Identifier, variable: Variable, definition: Expr) -> _DefExpr {
        _DefExpr {
            identifier: identifier,
            variable: variable,
            definition: definition,
        }
    }
}

impl Identify for _DefExpr {
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
pub struct Function(Object<_Function>);
pub type Functions = HashMap<Identifier, Function>;

impl Function {
    pub fn new(symbol: Symbol,
               formals: Variables,
               block: Option<BlockExpr>,
               ty: LambdaType)
               -> Function {
        Function(object!(_Function::new(symbol, formals, block, ty)))
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
struct _Function {
    symbol: Symbol,
    formals: Variables,
    block: Option<BlockExpr>,
    ty: LambdaType,
}

impl _Function {
    fn new(symbol: Symbol,
           formals: Variables,
           block: Option<BlockExpr>,
           ty: LambdaType)
           -> _Function {
        _Function {
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

impl Typedef for _Function {
    fn typedef(&self) -> Type {
        Type::Lambda(self.ty.clone())
    }
}

impl Identify for _Function {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for _Function {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct ItemExpr(Object<_ItemExpr>);

impl ItemExpr {
    pub fn new(identifier: Identifier, item: Item) -> ItemExpr {
        ItemExpr(object!(_ItemExpr::new(identifier, item)))
    }
}

impl Identify for ItemExpr {
    fn identify(&self) -> Identifier {
        object_proxy![self.0 => identify()]
    }
}

impl Symbolise for ItemExpr {
    fn symbolise(&self) -> Symbol {
        object_proxy![self.0 => symbolise()]
    }
}

impl Typedef for ItemExpr {
    fn typedef(&self) -> Type {
        object_proxy![self.0 => typedef()]
    }
}

#[derive(Clone)]
struct _ItemExpr {
    identifier: Identifier,
    item: Item,
}

impl _ItemExpr {
    fn new(identifier: Identifier, item: Item) -> _ItemExpr {
        _ItemExpr {
            identifier: identifier,
            item: item,
        }
    }
}

impl Identify for _ItemExpr {
    fn identify(&self) -> Identifier {
        item_proxy![*&self.item => identify()]
    }
}

impl Symbolise for _ItemExpr {
    fn symbolise(&self) -> Symbol {
        item_proxy![*&self.item => symbolise()]
    }
}

impl Typedef for _ItemExpr {
    fn typedef(&self) -> Type {
        item_proxy![*&self.item => typedef()]
    }
}

#[derive(Clone)]
pub enum Item {
    Function(Function),
    Variable(Variable),
}

///
#[derive(Clone)]
pub struct LambdaType(Object<_LambdaType>);

impl LambdaType {
    pub fn new(formals: Vec<Type>, ty: Type) -> LambdaType {
        LambdaType(object!(_LambdaType::new(formals, ty)))
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
pub struct _LambdaType {
    formals: Vec<Type>,
    ty: Type,
}

impl _LambdaType {
    fn new(formals: Vec<Type>, ty: Type) -> _LambdaType {
        _LambdaType {
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

impl Typedef for _LambdaType {
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
pub struct Module(Object<_Module>);

impl Module {
    pub fn new(symbol: Symbol, functions: Functions) -> Module {
        Module(object!(_Module::new(symbol, functions)))
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
pub struct _Module {
    // identification
    symbol: Symbol,

    // functions defined in this module
    functions: Functions,
}

impl _Module {
    fn new(symbol: Symbol, functions: Functions) -> _Module {
        _Module {
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

impl Identify for _Module {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for _Module {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct ProcessExpr(Object<_ProcessExpr>);

impl ProcessExpr {
    pub fn new(identifier: Identifier,
               prelude_exprs: Exprs,
               prelude_variables: Variables,
               body: Exprs)
               -> ProcessExpr {
        ProcessExpr(object!(_ProcessExpr::new(identifier,
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
struct _ProcessExpr {
    identifier: Identifier,
    prelude_exprs: Exprs,
    prelude_variables: Variables,
    body: Exprs,
}

impl _ProcessExpr {
    fn new(identifier: Identifier,
           prelude_exprs: Exprs,
           prelude_variables: Variables,
           body: Exprs)
           -> _ProcessExpr {
        _ProcessExpr {
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

impl Identify for _ProcessExpr {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

///
#[derive(Clone)]
pub struct ProcessJoinExpr(Object<_ProcessJoinExpr>);

impl ProcessJoinExpr {
    pub fn new(identifier: Identifier, process: ProcessExpr) -> ProcessJoinExpr {
        ProcessJoinExpr(object!(_ProcessJoinExpr::new(identifier, process)))
    }

    pub fn process(&self) -> ProcessExpr {
        object_proxy![self.0 => process().clone()]
    }
}

#[derive(Clone)]
struct _ProcessJoinExpr {
    identifier: Identifier,
    process: ProcessExpr,
}

impl _ProcessJoinExpr {
    fn new(identifier: Identifier, process: ProcessExpr) -> _ProcessJoinExpr {
        _ProcessJoinExpr {
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
pub struct PtrType(Object<_PtrType>);

impl PtrType {
    pub fn new(ty: Type) -> PtrType {
        PtrType(object!(_PtrType::new(ty)))
    }

    pub fn ty(&self) -> Type {
        object_proxy![self.0 => ty().clone()]
    }
}

#[derive(Clone)]
struct _PtrType {
    ty: Type,
}

impl _PtrType {
    fn new(ty: Type) -> _PtrType {
        _PtrType { ty: ty }
    }

    fn ty(&self) -> &Type {
        &self.ty
    }
}

///
#[derive(Clone)]
pub struct StructExpr(Object<_StructExpr>);

impl StructExpr {
    pub fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> StructExpr {
        StructExpr(object!(_StructExpr::new(identifier, elements, ty)))
    }
}

#[derive(Clone)]
struct _StructExpr {
    identifier: Identifier,
    elements: Exprs,
    ty: StructType,
}

impl _StructExpr {
    fn new(identifier: Identifier, elements: Exprs, ty: StructType) -> _StructExpr {
        _StructExpr {
            identifier: identifier,
            elements: elements,
            ty: ty,
        }
    }
}

///
#[derive(Clone)]
pub struct StructElementExpr(Object<_StructElementExpr>);

impl StructElementExpr {
    pub fn new(identifier: Identifier, target: Expr, index: u64) -> StructElementExpr {
        StructElementExpr(object!(_StructElementExpr::new(identifier, target, index)))
    }
}

#[derive(Clone)]
struct _StructElementExpr {
    identifier: Identifier,
    target: Expr,
    index: u64,
}

impl _StructElementExpr {
    fn new(identifier: Identifier, target: Expr, index: u64) -> _StructElementExpr {
        _StructElementExpr {
            identifier: identifier,
            target: target,
            index: index,
        }
    }
}

///
#[derive(Clone)]
pub struct StructType(Object<_StructType>);

impl StructType {
    pub fn new(elements: Variables) -> StructType {
        StructType(object!(_StructType::new(elements)))
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
struct _StructType {
    elements: Variables,
}

impl _StructType {
    fn new(elements: Variables) -> _StructType {
        _StructType { elements: elements }
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
pub struct Variable(Object<_Variable>);
pub type Variables = Vec<Variable>;

impl Variable {
    pub fn new(symbol: Symbol, ty: Type) -> Variable {
        Variable(object!(_Variable::new(symbol, ty)))
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
struct _Variable {
    // identification
    symbol: Symbol,

    // type
    ty: Type,
}

impl _Variable {
    fn new(symbol: Symbol, ty: Type) -> _Variable {
        _Variable {
            symbol: symbol,
            ty: ty,
        }
    }
}

impl Typedef for _Variable {
    fn typedef(&self) -> Type {
        self.ty.clone()
    }
}

impl Identify for _Variable {
    fn identify(&self) -> Identifier {
        self.symbol.identify()
    }
}

impl Symbolise for _Variable {
    fn symbolise(&self) -> Symbol {
        self.symbol.clone()
    }
}

///
#[derive(Clone)]
pub struct VoidExpr(Object<_VoidExpr>);

impl VoidExpr {
    pub fn new(identifier: Identifier) -> VoidExpr {
        VoidExpr(object!(_VoidExpr::new(identifier)))
    }
}

#[derive(Clone)]
struct _VoidExpr {
    identifier: Identifier,
}

impl _VoidExpr {
    fn new(identifier: Identifier) -> _VoidExpr {
        _VoidExpr { identifier: identifier }
    }
}