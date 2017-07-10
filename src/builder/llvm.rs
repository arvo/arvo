extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use self::llvm_sys::target::*;

use super::super::identifier::{Identifier, Identify, Name, Symbol, Symbolise};
use super::super::noir::*;
use super::super::noir::context::*;
use super::super::noir::prelude::*;
use super::super::noir::runtime::*;

use std::collections::HashMap;
use std::env;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::process::{Command, Stdio};
use std::ptr;

pub struct LLVMBuilder {
    name: String,
    context: Context,
    llvm_context: LLVMContextRef,
    llvm_module: LLVMModuleRef,
    llvm_builder: LLVMBuilderRef,
    llvm_values: HashMap<Identifier, LLVMValueRef>,
    llvm_types: HashMap<Identifier, LLVMTypeRef>,
    llvm_current_function: Option<LLVMValueRef>,
    llvm_current_block: Option<LLVMBasicBlockRef>,
}

impl LLVMBuilder {
    pub fn new<S>(name: S, context: Context) -> LLVMBuilder
        where S: Clone + Into<String>
    {
        unsafe {
            // initialise LLVM
            let llvm_context = LLVMContextCreate();
            let llvm_module_name = llvm_string(name.clone().into());
            let llvm_module = LLVMModuleCreateWithName(llvm_module_name.as_ptr());
            LLVMBuilder {
                name: name.into(),
                context: context,
                llvm_context: llvm_context,
                llvm_module: llvm_module,
                llvm_builder: LLVMCreateBuilderInContext(llvm_context),
                llvm_values: HashMap::new(),
                llvm_types: HashMap::new(),
                llvm_current_function: None,
                llvm_current_block: None,
            }
        }
    }

    pub fn codegen_block_expr<S>(&mut self,
                                 block_name: S,
                                 block_expr: &BlockExpr)
                                 -> Option<LLVMValueRef>
        where S: Into<String>
    {
        let llvm_name = llvm_string(block_name);
        let block = unsafe {
            LLVMAppendBasicBlockInContext(self.llvm_context, self.llvm_current_function.expect("expected Some(LLVMValueRef) for None"), llvm_name.as_ptr())
        };
        unsafe {
            LLVMPositionBuilderAtEnd(self.llvm_builder, block);
        };

        self.codegen_exprs(&block_expr.prelude);
        self.codegen_exprs(&block_expr.body);
        self.codegen_exprs(&block_expr.epilogue);

        // TODO: codegen block expression return

        None
    }

    pub fn codegen_call_expr(&mut self, call_expr: &CallExpr) -> LLVMValueRef {
        let llvm_name = llvm_string("");
        let llvm_target = self.codegen_expr(&call_expr.target);
        let mut llvm_arguments = self.codegen_exprs(&call_expr.arguments).into_iter().map(|arg| arg.unwrap()).collect::<Vec<_>>();
        unsafe {
            LLVMBuildCall(self.llvm_builder,
                          llvm_target.unwrap(),
                          llvm_arguments.as_mut_ptr(),
                          llvm_arguments.len() as u32,
                          llvm_name.as_ptr())
        }
    }

    pub fn codegen_function(&mut self, function: &Function) -> LLVMValueRef {
        self.add_or_get_function_profile(function);

        unsafe {
            // Get llvm function
            let llvm_name = llvm_string(function.mangled_name.clone());
            let llvm_fn = LLVMGetNamedFunction(self.llvm_module, llvm_name.as_ptr());
            if llvm_fn == ptr::null_mut() {
                panic!("error: use of undefined function '{}'",
                       function.identify().name);
            }
            // Check if the function has been defined
            if LLVMCountBasicBlocks(llvm_fn) != 0 {
                panic!("error: redefinition of function '{}'",
                       function.identify().name);
            }
            // Check if the arguments match any other declarations
            let n_args = LLVMCountParams(llvm_fn) as usize;
            if n_args != 0 && n_args != function.formals.len() {
                panic!("error: redefinition of function '{}' with different arguments",
                       function.identify().name);
            }

            // Build the function definition
            for (i, formal) in function.formals.iter().enumerate() {
                let llvm_param = LLVMGetParam(llvm_fn, i as u32);
                let llvm_name = CString::new(formal.identify().name.clone()).unwrap();
                LLVMSetValueName(llvm_param, llvm_name.as_ptr());
                self.llvm_values
                    .insert(formal.identify().clone(), llvm_param);
            }

            // Build the function body definition
            if let Some(ref block_expr) = function.body {
                let llvm_restore_point = LLVMGetInsertBlock(self.llvm_builder);
                LLVMClearInsertionPosition(self.llvm_builder);

                // Build the block
                self.llvm_current_function = Some(llvm_fn);
                let llvm_ret = self.codegen_block_expr("entry", block_expr);

                // Build the return type
                if let Some(llvm_ret) = llvm_ret {
                    let llvm_ret_type = LLVMTypeOf(llvm_ret);
                    if llvm_ret_type == LLVMVoidType() {
                        LLVMBuildRetVoid(self.llvm_builder);
                    } else {
                        LLVMBuildRet(self.llvm_builder, llvm_ret);
                    }
                } else {
                    LLVMBuildRetVoid(self.llvm_builder);
                }

                LLVMPositionBuilderAtEnd(self.llvm_builder, llvm_restore_point);
            }

            llvm_fn
        }
    }

    pub fn add_or_get_function_profile(&mut self, function: &Function) -> LLVMValueRef {
        if let Some(node) = self.llvm_values.get(&function.identify()) {
            return node.clone();
        }
        unsafe {
            // check if the function exists
            let llvm_name = llvm_string(function.symbolise().name());
            if LLVMGetNamedFunction(self.llvm_module, llvm_name.as_ptr()) != ptr::null_mut() {
                panic!("duplicate declaration of function '{}'",
                       function.symbolise().name());
            }
            // build the function profile
            let lambda_type = function.lambda_type();
            let llvm_fn = LLVMFunctionType(self.codegen_type(&lambda_type.ret),
                                           self.codegen_types(&lambda_type.formals).as_mut_ptr(),
                                           function.formals.len() as u32,
                                           0);
            // add the function to the module
            let llvm_fn = LLVMAddFunction(self.llvm_module, llvm_name.as_ptr(), llvm_fn);
            self.llvm_values.insert(function.identify(), llvm_fn);
            llvm_fn
        }
    }

    pub fn codegen_item_expr(&mut self, item_expr: &ItemExpr) -> LLVMValueRef {
        self.llvm_values.get(&item_expr.symbolise().identify()).expect("use of undefined value").clone()
    }

    pub fn codegen_lambda_type(&mut self, lambda_type: &LambdaType) -> LLVMTypeRef {
        unsafe {
            LLVMPointerType(
                LLVMFunctionType(self.codegen_type(&lambda_type.ret),
                    lambda_type.formals
                        .iter()
                        .map(|formal| self.codegen_type(formal))
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    lambda_type.formals.len() as u32,
                0), 0
            )
        }
    }

    pub fn codegen_literal_expr(&mut self, literal_expr: &LiteralExpr) -> LLVMValueRef {
        self.codegen_literal(&literal_expr.literal)
    }

    pub fn codegen_process_expr(&mut self, process_expr: &ProcessExpr) -> LLVMValueRef {
        // create a function for the process
        let process_function = process_expr.function();
        self.codegen_function(&process_function);
        // create a function call to the runtime to launch the process function
        let process_call_expr = CallExpr::new(
            ItemExpr::new(self.context.runtime.process_fn.clone()),
            vec![
                ItemExpr::new(process_function)
            ]
        );
        let llvm_call_expr = self.codegen_call_expr(&process_call_expr);
        // add the process to the nodes
        self.llvm_values
            .insert(process_expr.identify(), llvm_call_expr.clone());
        llvm_call_expr
    }

    pub fn codegen_process_join_expr(&mut self,
                                     process_join_expr: &ProcessJoinExpr)
                                     -> LLVMValueRef {
        // create a call target from the process join function
        let llvm_target = self.llvm_values
            .get(&self.context.runtime.process_join_fn.identify())
            .unwrap()
            .clone();
        // create arguments from the process being joined
        let mut llvm_arguments = vec![self.llvm_values
                                          .get(&process_join_expr.process_expr.identify())
                                          .unwrap()
                                          .clone()];
        unsafe {
            // create the function call
            let llvm_name = llvm_string("");
            LLVMBuildCall(self.llvm_builder,
                          llvm_target,
                          llvm_arguments.as_mut_ptr(),
                          llvm_arguments.len() as u32,
                          llvm_name.as_ptr())
        }
    }

    pub fn codegen_ptr_type(&mut self, ptr_type: &PtrType) -> LLVMTypeRef {
        unsafe { LLVMPointerType(self.codegen_type(&ptr_type.inner)    , 0) }
    }

    pub fn codegen_ref_type(&mut self, ref_type: &RefType) -> LLVMTypeRef {
        unsafe { LLVMPointerType(self.codegen_type(&ref_type.inner)    , 0) }
    }


    pub fn codegen_void_expr(&mut self, void_expr: &VoidExpr) -> Option<LLVMValueRef> {
        None
    }

    pub fn codegen_literal(&mut self, literal: &Literal) -> LLVMValueRef {
        unsafe {
            match *literal {
                Literal::Bool(ref value) => {
                    if value.clone() {
                        LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::Bool), 1, 0)
                    } else {
                        LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::Bool), 0, 0)
                    }
                }
                Literal::Char(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::Char),
                                 value.clone() as u64,
                                 0)
                }
                Literal::F32(ref value) => {
                    LLVMConstReal(self.codegen_primitive_type(&PrimitiveType::F32),
                                  value.clone() as f64)
                }
                Literal::F64(ref value) => {
                    LLVMConstReal(self.codegen_primitive_type(&PrimitiveType::F64),
                                  value.clone() as f64)
                }
                Literal::I8(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::I8),
                                 value.clone() as u64,
                                 0)
                }
                Literal::I16(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::I16),
                                 value.clone() as u64,
                                 0)
                }
                Literal::I32(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::I32),
                                 value.clone() as u64,
                                 0)
                }
                Literal::I64(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::I64),
                                 value.clone() as u64,
                                 0)
                }
                Literal::Str(ref value) => {
                    let string = CString::new(value.clone()).unwrap();
                    let string_bytes = string.as_bytes_with_nul();
                    let llvm_string = llvm_string(value.clone());
                    let llvm_string_global = LLVMBuildGlobalStringPtr(self.llvm_builder,
                                                                      string_bytes.as_ptr() as
                                                                      *const _,
                                                                      llvm_string.as_ptr());
                    LLVMConstGEP(llvm_string_global,
                                 vec![LLVMConstInt(LLVMInt32Type(), 0, 0)].as_mut_ptr(),
                                 1)
                }
                Literal::U8(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::U8),
                                 value.clone() as u64,
                                 0)
                }
                Literal::U16(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::U16),
                                 value.clone() as u64,
                                 0)
                }
                Literal::U32(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::U32),
                                 value.clone() as u64,
                                 0)
                }
                Literal::U64(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::U64),
                                 value.clone() as u64,
                                 0)
                }
                Literal::USize(ref value) => {
                    LLVMConstInt(self.codegen_primitive_type(&PrimitiveType::USize),
                                 value.clone() as u64,
                                 0)
                }
            }
        }
    }

    pub fn codegen_primitive_type(&mut self, primitive_type: &PrimitiveType) -> LLVMTypeRef {
        unsafe {
            match *primitive_type {
                PrimitiveType::Bool => LLVMInt1Type(),
                PrimitiveType::Char => LLVMInt32Type(),
                PrimitiveType::F32 => LLVMFloatType(),
                PrimitiveType::F64 => LLVMDoubleType(),
                PrimitiveType::I8 => LLVMInt8Type(),
                PrimitiveType::I16 => LLVMInt16Type(),
                PrimitiveType::I32 => LLVMInt32Type(),
                PrimitiveType::I64 => LLVMInt64Type(),
                PrimitiveType::Str => {
                    // Strings are pointers to an object, which for now, is
                    // considered to be a null terminated array of utf-8
                    // encoded bytes 
                    LLVMPointerType(self.codegen_primitive_type(&PrimitiveType::I8), 0)
                }
                PrimitiveType::U8 => LLVMInt8Type(),
                PrimitiveType::U16 => LLVMInt16Type(),
                PrimitiveType::U32 => LLVMInt32Type(),
                PrimitiveType::U64 => LLVMInt64Type(),
                PrimitiveType::USize => {
                    let llvm_ptr_size = LLVMPointerSize(LLVMGetModuleDataLayout(self.llvm_module));
                    LLVMIntType(llvm_ptr_size)
                }
                PrimitiveType::Void => LLVMVoidType(),
            }
        }
    }

    pub fn codegen_expr(&mut self, expr: &Expr) -> Option<LLVMValueRef> {
        match *expr {
            Expr::Block(ref expr) => self.codegen_block_expr("", expr),
            Expr::Call(ref expr) => Some(self.codegen_call_expr(expr)),
            Expr::Process(ref expr) => Some(self.codegen_process_expr(expr)),
            Expr::ProcessJoin(ref expr) => Some(self.codegen_process_join_expr(expr)),
            Expr::Item(ref expr) => Some(self.codegen_item_expr(expr)),
            Expr::Literal(ref expr) => Some(self.codegen_literal_expr(expr)),
            Expr::Void(ref expr) => self.codegen_void_expr(expr),
            _ => unimplemented!(),
        }
    }

    pub fn codegen_exprs(&mut self, exprs: &Exprs) -> Vec<Option<LLVMValueRef>> {
        exprs
            .iter()
            .map(|expr| self.codegen_expr(expr))
            .collect()
    }

    pub fn codegen_type(&mut self, ty: &Type) -> LLVMTypeRef {
        match *ty {
            Type::Lambda(ref ty) => self.codegen_lambda_type(ty),
            Type::Primitive(ref ty) => self.codegen_primitive_type(ty),
            Type::Ptr(ref ty) => self.codegen_ptr_type(ty),
            Type::Ref(ref ty) => self.codegen_ref_type(ty),
            _ => unimplemented!(),
        }
    }

    pub fn codegen_types(&mut self, types: &Types) -> Vec<LLVMTypeRef> {
        types.iter().map(|ty| self.codegen_type(ty)).collect()
    }

    pub fn codegen_runtime(&mut self) {
        for function in self.context.runtime.functions() {
            self.codegen_function(&function);
        }
    }

    pub fn codegen_prelude(&mut self) {
        for function in self.context.prelude.functions() {
            self.codegen_function(&function);
        }
    }

    pub fn dump(&self) {
        llvm_dump_module(self.llvm_module);
    }

    pub fn dump_to_string(&self) -> String {
        llvm_dump_module_to_string(self.llvm_module)
    }

    pub fn build(&self) {
        // dump IR to string
        let bytecode = self.dump_to_string();

        // link using llc
        let mut ld_out = self.name.clone();
        ld_out.push_str(".o");
        let ld = match Command::new("llc-3.9")
                  .args(&["-filetype=obj", "-o", ld_out.as_str()])
                  .stdin(Stdio::piped())
                  .spawn() {
            Ok(ld) => ld,
            Err(err) => panic!(err.to_string()),
        };
        match ld.stdin.unwrap().write_all(bytecode.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!(err.to_string()),
        };

        // compile using clang
        let clang = Command::new("clang")
            .args(&[ld_out.as_str(),
                    &format!("{}/libruntime.a", env::var("ARVO_LIB_PATH").unwrap()),
                    &format!("{}/libprelude.a", env::var("ARVO_LIB_PATH").unwrap()),
                    "-pthread",
                    "-ldl",
                    "-o",
                    self.name.as_ref()])
            .output()
            .unwrap_or_else(|err| panic!("{}", err));
        if clang.status.success() {
            print!("{}", String::from_utf8_lossy(&clang.stdout));
        } else {
            panic!("{}", String::from_utf8_lossy(&clang.stderr));
        }
    }
}

///
#[derive(Clone)]
pub enum LLVMNode {
    Type(LLVMTypeRef),
    Value(LLVMValueRef),
    Nil,
}

impl Default for LLVMNode {
    fn default() -> LLVMNode {
        LLVMNode::Nil
    }
}

impl From<LLVMTypeRef> for LLVMNode {
    fn from(ty: LLVMTypeRef) -> LLVMNode {
        LLVMNode::Type(ty)
    }
}

impl From<LLVMValueRef> for LLVMNode {
    fn from(value: LLVMValueRef) -> LLVMNode {
        LLVMNode::Value(value)
    }
}

impl From<LLVMNode> for LLVMTypeRef {
    fn from(node: LLVMNode) -> LLVMTypeRef {
        match node {
            LLVMNode::Nil => panic!("expected LLVMNode::Type got LLVMNode::Nil"),
            LLVMNode::Type(ty) => ty,
            LLVMNode::Value(..) => panic!("expected LLVMNode::Type got LLVMNode::Value"),
        }
    }
}

impl From<LLVMNode> for LLVMValueRef {
    fn from(node: LLVMNode) -> LLVMValueRef {
        match node {
            LLVMNode::Nil => panic!("expected LLVMNode::Value got LLVMNode::Nil"),
            LLVMNode::Type(..) => panic!("expected LLVMNode::Value got LLVMNode::Type"),
            LLVMNode::Value(value) => value,
        }
    }
}

fn llvm_string<S: Into<String>>(string: S) -> CString {
    CString::new(string.into()).unwrap()
}

fn llvm_dump_module(module: LLVMModuleRef) {
    unsafe {
        println!("{}",
                 CStr::from_ptr(LLVMPrintModuleToString(module))
                     .to_string_lossy()
                     .into_owned());
    }
}

fn llvm_dump_module_to_string(module: LLVMModuleRef) -> String {
    unsafe {
        CStr::from_ptr(LLVMPrintModuleToString(module))
            .to_string_lossy()
            .into_owned()
    }
}