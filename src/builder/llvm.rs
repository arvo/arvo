extern crate llvm_sys;

// use self::llvm_sys::core::*;
// use self::llvm_sys::prelude::*;
// use self::llvm_sys::target::*;

// use id::{Identifier, Identify, Name, Symbol, Symbolise};
// use noir::*;
// use noir::prelude::*;
// use noir::runtime::*;
// use noir::visitor::*;

// use std::collections::HashMap;
// use std::ffi::{CStr, CString};
// use std::io::Write;
// use std::process::{Command, Stdio};
// use std::ptr::null_mut;

// pub struct LLVMBuilder {
//     name: String,
//     context: LLVMContextRef,
//     module: LLVMModuleRef,
//     builder: LLVMBuilderRef,
//     nodes: HashMap<Identifier, LLVMNode>,
//     noir_context: Context,
// }

// impl LLVMBuilder {
//     /// Create a new `LLVMBuilder`.
//     pub fn new<N>(name: N, noir_context: Context) -> LLVMBuilder
//         where N: Clone + Into<String>
//     {
//         let mut builder = unsafe {
//             // initialise LLVM
//             let context = LLVMContextCreate();
//             let module_name = llvm_string(name.clone());
//             let module = LLVMModuleCreateWithName(module_name.as_ptr() as *const _);
//             let builder = LLVMCreateBuilderInContext(context);
//             LLVMBuilder {
//                 name: name.into(),
//                 context: context,
//                 module: module,
//                 builder: builder,
//                 nodes: HashMap::new(),
//                 noir_context: noir_context,
//             }
//         };
//         // initialise prelude
//         let libprelude_add_f32_fn = builder.noir_context().prelude().add_f32_fn();
//         let libprelude_add_f64_fn = builder.noir_context().prelude().add_f64_fn();
//         let libprelude_add_i8_fn = builder.noir_context().prelude().add_i8_fn();
//         let libprelude_add_i16_fn = builder.noir_context().prelude().add_i16_fn();
//         let libprelude_add_i32_fn = builder.noir_context().prelude().add_i32_fn();
//         let libprelude_add_i64_fn = builder.noir_context().prelude().add_i64_fn();
//         let libprelude_add_u8_fn = builder.noir_context().prelude().add_u8_fn();
//         let libprelude_add_u16_fn = builder.noir_context().prelude().add_u16_fn();
//         let libprelude_add_u32_fn = builder.noir_context().prelude().add_u32_fn();
//         let libprelude_add_u64_fn = builder.noir_context().prelude().add_u64_fn();
//         let libprelude_add_usize_fn = builder.noir_context().prelude().add_usize_fn();
//         let libprelude_div_f32_fn = builder.noir_context().prelude().div_f32_fn();
//         let libprelude_div_f64_fn = builder.noir_context().prelude().div_f64_fn();
//         let libprelude_div_i8_fn = builder.noir_context().prelude().div_i8_fn();
//         let libprelude_div_i16_fn = builder.noir_context().prelude().div_i16_fn();
//         let libprelude_div_i32_fn = builder.noir_context().prelude().div_i32_fn();
//         let libprelude_div_i64_fn = builder.noir_context().prelude().div_i64_fn();
//         let libprelude_div_u8_fn = builder.noir_context().prelude().div_u8_fn();
//         let libprelude_div_u16_fn = builder.noir_context().prelude().div_u16_fn();
//         let libprelude_div_u32_fn = builder.noir_context().prelude().div_u32_fn();
//         let libprelude_div_u64_fn = builder.noir_context().prelude().div_u64_fn();
//         let libprelude_div_usize_fn = builder.noir_context().prelude().div_usize_fn();
//         let libprelude_mul_f32_fn = builder.noir_context().prelude().mul_f32_fn();
//         let libprelude_mul_f64_fn = builder.noir_context().prelude().mul_f64_fn();
//         let libprelude_mul_i8_fn = builder.noir_context().prelude().mul_i8_fn();
//         let libprelude_mul_i16_fn = builder.noir_context().prelude().mul_i16_fn();
//         let libprelude_mul_i32_fn = builder.noir_context().prelude().mul_i32_fn();
//         let libprelude_mul_i64_fn = builder.noir_context().prelude().mul_i64_fn();
//         let libprelude_mul_u8_fn = builder.noir_context().prelude().mul_u8_fn();
//         let libprelude_mul_u16_fn = builder.noir_context().prelude().mul_u16_fn();
//         let libprelude_mul_u32_fn = builder.noir_context().prelude().mul_u32_fn();
//         let libprelude_mul_u64_fn = builder.noir_context().prelude().mul_u64_fn();
//         let libprelude_mul_usize_fn = builder.noir_context().prelude().mul_usize_fn();
//         let libprelude_sub_f32_fn = builder.noir_context().prelude().sub_f32_fn();
//         let libprelude_sub_f64_fn = builder.noir_context().prelude().sub_f64_fn();
//         let libprelude_sub_i8_fn = builder.noir_context().prelude().sub_i8_fn();
//         let libprelude_sub_i16_fn = builder.noir_context().prelude().sub_i16_fn();
//         let libprelude_sub_i32_fn = builder.noir_context().prelude().sub_i32_fn();
//         let libprelude_sub_i64_fn = builder.noir_context().prelude().sub_i64_fn();
//         let libprelude_sub_u8_fn = builder.noir_context().prelude().sub_u8_fn();
//         let libprelude_sub_u16_fn = builder.noir_context().prelude().sub_u16_fn();
//         let libprelude_sub_u32_fn = builder.noir_context().prelude().sub_u32_fn();
//         let libprelude_sub_u64_fn = builder.noir_context().prelude().sub_u64_fn();
//         let libprelude_sub_usize_fn = builder.noir_context().prelude().sub_usize_fn();
//         let libprelude_concat_string_bool_fn = builder.noir_context().prelude().concat_string_bool_fn();
//         let libprelude_concat_string_char_fn = builder.noir_context().prelude().concat_string_char_fn();
//         let libprelude_concat_string_f32_fn = builder.noir_context().prelude().concat_string_f32_fn();
//         let libprelude_concat_string_f64_fn = builder.noir_context().prelude().concat_string_f64_fn();
//         let libprelude_concat_string_i8_fn = builder.noir_context().prelude().concat_string_i8_fn();
//         let libprelude_concat_string_i16_fn = builder.noir_context().prelude().concat_string_i16_fn();
//         let libprelude_concat_string_i32_fn = builder.noir_context().prelude().concat_string_i32_fn();
//         let libprelude_concat_string_i64_fn = builder.noir_context().prelude().concat_string_i64_fn();
//         let libprelude_concat_string_string_fn =
//             builder.noir_context().prelude().concat_string_string_fn();
//         let libprelude_concat_string_u8_fn = builder.noir_context().prelude().concat_string_u8_fn();
//         let libprelude_concat_string_u16_fn = builder.noir_context().prelude().concat_string_u16_fn();
//         let libprelude_concat_string_u32_fn = builder.noir_context().prelude().concat_string_u32_fn();
//         let libprelude_concat_string_u64_fn = builder.noir_context().prelude().concat_string_u64_fn();
//         let libprelude_concat_string_usize_fn = builder.noir_context().prelude().concat_string_usize_fn();
//         let libprelude_write_bool_fn = builder.noir_context().prelude().write_bool_fn();
//         let libprelude_write_char_fn = builder.noir_context().prelude().write_char_fn();
//         let libprelude_write_f32_fn = builder.noir_context().prelude().write_f32_fn();
//         let libprelude_write_f64_fn = builder.noir_context().prelude().write_f64_fn();
//         let libprelude_write_i8_fn = builder.noir_context().prelude().write_i8_fn();
//         let libprelude_write_i16_fn = builder.noir_context().prelude().write_i16_fn();
//         let libprelude_write_i32_fn = builder.noir_context().prelude().write_i32_fn();
//         let libprelude_write_i64_fn = builder.noir_context().prelude().write_i64_fn();
//         let libprelude_write_string_fn = builder.noir_context().prelude().write_string_fn();
//         let libprelude_write_u8_fn = builder.noir_context().prelude().write_u8_fn();
//         let libprelude_write_u16_fn = builder.noir_context().prelude().write_u16_fn();
//         let libprelude_write_u32_fn = builder.noir_context().prelude().write_u32_fn();
//         let libprelude_write_u64_fn = builder.noir_context().prelude().write_u64_fn();
//         let libprelude_write_usize_fn = builder.noir_context().prelude().write_usize_fn();
//         let libprelude_writeln_bool_fn = builder.noir_context().prelude().writeln_bool_fn();
//         let libprelude_writeln_char_fn = builder.noir_context().prelude().writeln_char_fn();
//         let libprelude_writeln_f32_fn = builder.noir_context().prelude().writeln_f32_fn();
//         let libprelude_writeln_f64_fn = builder.noir_context().prelude().writeln_f64_fn();
//         let libprelude_writeln_i8_fn = builder.noir_context().prelude().writeln_i8_fn();
//         let libprelude_writeln_i16_fn = builder.noir_context().prelude().writeln_i16_fn();
//         let libprelude_writeln_i32_fn = builder.noir_context().prelude().writeln_i32_fn();
//         let libprelude_writeln_i64_fn = builder.noir_context().prelude().writeln_i64_fn();
//         let libprelude_writeln_string_fn = builder.noir_context().prelude().writeln_string_fn();
//         let libprelude_writeln_u8_fn = builder.noir_context().prelude().writeln_u8_fn();
//         let libprelude_writeln_u16_fn = builder.noir_context().prelude().writeln_u16_fn();
//         let libprelude_writeln_u32_fn = builder.noir_context().prelude().writeln_u32_fn();
//         let libprelude_writeln_u64_fn = builder.noir_context().prelude().writeln_u64_fn();
//         let libprelude_writeln_usize_fn = builder.noir_context().prelude().writeln_usize_fn();
//         builder.visit_function(libprelude_add_f32_fn);
//         builder.visit_function(libprelude_add_f64_fn);
//         builder.visit_function(libprelude_add_i8_fn);
//         builder.visit_function(libprelude_add_i16_fn);
//         builder.visit_function(libprelude_add_i32_fn);
//         builder.visit_function(libprelude_add_i64_fn);
//         builder.visit_function(libprelude_add_u8_fn);
//         builder.visit_function(libprelude_add_u16_fn);
//         builder.visit_function(libprelude_add_u32_fn);
//         builder.visit_function(libprelude_add_u64_fn);
//         builder.visit_function(libprelude_add_usize_fn);
//         builder.visit_function(libprelude_div_f32_fn);
//         builder.visit_function(libprelude_div_f64_fn);
//         builder.visit_function(libprelude_div_i8_fn);
//         builder.visit_function(libprelude_div_i16_fn);
//         builder.visit_function(libprelude_div_i32_fn);
//         builder.visit_function(libprelude_div_i64_fn);
//         builder.visit_function(libprelude_div_u8_fn);
//         builder.visit_function(libprelude_div_u16_fn);
//         builder.visit_function(libprelude_div_u32_fn);
//         builder.visit_function(libprelude_div_u64_fn);
//         builder.visit_function(libprelude_div_usize_fn);
//         builder.visit_function(libprelude_mul_f32_fn);
//         builder.visit_function(libprelude_mul_f64_fn);
//         builder.visit_function(libprelude_mul_i8_fn);
//         builder.visit_function(libprelude_mul_i16_fn);
//         builder.visit_function(libprelude_mul_i32_fn);
//         builder.visit_function(libprelude_mul_i64_fn);
//         builder.visit_function(libprelude_mul_u8_fn);
//         builder.visit_function(libprelude_mul_u16_fn);
//         builder.visit_function(libprelude_mul_u32_fn);
//         builder.visit_function(libprelude_mul_u64_fn);
//         builder.visit_function(libprelude_mul_usize_fn);
//         builder.visit_function(libprelude_sub_f32_fn);
//         builder.visit_function(libprelude_sub_f64_fn);
//         builder.visit_function(libprelude_sub_i8_fn);
//         builder.visit_function(libprelude_sub_i16_fn);
//         builder.visit_function(libprelude_sub_i32_fn);
//         builder.visit_function(libprelude_sub_i64_fn);
//         builder.visit_function(libprelude_sub_u8_fn);
//         builder.visit_function(libprelude_sub_u16_fn);
//         builder.visit_function(libprelude_sub_u32_fn);
//         builder.visit_function(libprelude_sub_u64_fn);
//         builder.visit_function(libprelude_sub_usize_fn);
//         builder.visit_function(libprelude_concat_string_bool_fn);
//         builder.visit_function(libprelude_concat_string_char_fn);
//         builder.visit_function(libprelude_concat_string_f32_fn);
//         builder.visit_function(libprelude_concat_string_f64_fn);
//         builder.visit_function(libprelude_concat_string_i8_fn);
//         builder.visit_function(libprelude_concat_string_i16_fn);
//         builder.visit_function(libprelude_concat_string_i32_fn);
//         builder.visit_function(libprelude_concat_string_i64_fn);
//         builder.visit_function(libprelude_concat_string_string_fn);
//         builder.visit_function(libprelude_concat_string_u8_fn);
//         builder.visit_function(libprelude_concat_string_u16_fn);
//         builder.visit_function(libprelude_concat_string_u32_fn);
//         builder.visit_function(libprelude_concat_string_u64_fn);
//         builder.visit_function(libprelude_concat_string_usize_fn);
//         builder.visit_function(libprelude_write_bool_fn);
//         builder.visit_function(libprelude_write_char_fn);
//         builder.visit_function(libprelude_write_f32_fn);
//         builder.visit_function(libprelude_write_f64_fn);
//         builder.visit_function(libprelude_write_i8_fn);
//         builder.visit_function(libprelude_write_i16_fn);
//         builder.visit_function(libprelude_write_i32_fn);
//         builder.visit_function(libprelude_write_i64_fn);
//         builder.visit_function(libprelude_write_string_fn);
//         builder.visit_function(libprelude_write_u8_fn);
//         builder.visit_function(libprelude_write_u16_fn);
//         builder.visit_function(libprelude_write_u32_fn);
//         builder.visit_function(libprelude_write_u64_fn);
//         builder.visit_function(libprelude_write_usize_fn);
//         builder.visit_function(libprelude_writeln_bool_fn);
//         builder.visit_function(libprelude_writeln_char_fn);
//         builder.visit_function(libprelude_writeln_f32_fn);
//         builder.visit_function(libprelude_writeln_f64_fn);
//         builder.visit_function(libprelude_writeln_i8_fn);
//         builder.visit_function(libprelude_writeln_i16_fn);
//         builder.visit_function(libprelude_writeln_i32_fn);
//         builder.visit_function(libprelude_writeln_i64_fn);
//         builder.visit_function(libprelude_writeln_string_fn);
//         builder.visit_function(libprelude_writeln_u8_fn);
//         builder.visit_function(libprelude_writeln_u16_fn);
//         builder.visit_function(libprelude_writeln_u32_fn);
//         builder.visit_function(libprelude_writeln_u64_fn);
//         builder.visit_function(libprelude_writeln_usize_fn);
//         // initialise runtime
//         let libruntime_process_fn = builder.noir_context().runtime().process_fn();
//         let libruntime_process_join_fn = builder.noir_context().runtime().process_join_fn();
//         builder.visit_function(libruntime_process_fn);
//         builder.visit_function(libruntime_process_join_fn);
//         // return
//         builder
//     }

//     pub fn build(&self) {
//         // dump IR to string
//         let bytecode = self.dump_to_string();

//         // link using llc
//         let mut ld_out = self.name.clone();
//         ld_out.push_str(".o");
//         let ld = match Command::new("llc-3.9")
//                   .args(&["-filetype=obj", "-o", ld_out.as_str()])
//                   .stdin(Stdio::piped())
//                   .spawn() {
//             Ok(ld) => ld,
//             Err(err) => panic!(err.to_string()),
//         };
//         match ld.stdin.unwrap().write_all(bytecode.as_bytes()) {
//             Ok(_) => (),
//             Err(err) => panic!(err.to_string()),
//         };

//         // compile using clang
//         let clang = Command::new("clang")
//             .args(&[ld_out.as_str(),
//                     "../../bin/lib/libprelude.a", // TODO: these are relative to the library and should be changed
//                     "../../bin/lib/libruntime.a",
//                     "-pthread",
//                     "-ldl",
//                     "-o",
//                     self.name.as_ref()])
//             .output()
//             .unwrap_or_else(|err| panic!("{}", err));
//         if clang.status.success() {
//             print!("{}", String::from_utf8_lossy(&clang.stdout));
//         } else {
//             panic!("{}", String::from_utf8_lossy(&clang.stderr));
//         }
//     }

//     pub fn dump(&self) {
//         llvm_dump_module(self.module);
//     }

//     pub fn dump_to_string(&self) -> String {
//         llvm_dump_module_to_string(self.module)
//     }

//     pub fn add_or_get_function_profile(&mut self, function: Function) -> LLVMValueRef {
//         if let Some(node) = self.nodes.get(&function.identify()) {
//             return LLVMValueRef::from(node.clone());
//         }
//         unsafe {
//             // check if the function exists
//             let llvm_name = llvm_string(function.symbolise().name());
//             if LLVMGetNamedFunction(self.module, llvm_name.as_ptr()) != null_mut() {
//                 panic!("duplicate declaration of function '{}'",
//                        function.symbolise().name());
//             }
//             // build the function profile
//             let llvm_fn = LLVMFunctionType(LLVMTypeRef::from(self.visit_type(function.ty().typedef())),
//                              function.ty()
//                                  .map_formals(|formal| LLVMTypeRef::from(self.visit_type(formal.clone())))
//                                  .as_mut_ptr(),
//                              function.num_formals() as u32,
//                              0);
//             // add the function to the module
//             let llvm_fn = LLVMAddFunction(self.module, llvm_name.as_ptr(), llvm_fn);
//             self.nodes
//                 .insert(function.identify(), LLVMNode::from(llvm_fn));
//             llvm_fn
//         }
//     }

//     pub fn noir_context(&self) -> &Context {
//         &self.noir_context
//     }
// }

// impl Visitor for LLVMBuilder {
//     type Return = LLVMNode;

//     fn visit_block_expr(&mut self, block_expr: BlockExpr) -> Self::Return {
//         block_expr.for_body_exprs(|expr| self.visit_expr(expr.clone()));
//         let llvm_ret = self.visit_expr(block_expr.ret());
//         block_expr.for_epilogue_exprs(|expr| self.visit_expr(expr.clone()));
//         llvm_ret
//     }

//     fn visit_bool_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt1Type()) }
//     }

//     fn visit_call_expr(&mut self, call_expr: CallExpr) -> Self::Return {
//         let llvm_name = llvm_string("");
//         let llvm_target = LLVMValueRef::from(self.visit_expr(call_expr.target()));
//         let mut llvm_arguments =
//             call_expr.map_arguments(|expr| LLVMValueRef::from(self.visit_expr(expr.clone())));
//         unsafe {
//             LLVMNode::from(LLVMBuildCall(self.builder,
//                                          llvm_target,
//                                          llvm_arguments.as_mut_ptr(),
//                                          llvm_arguments.len() as u32,
//                                          llvm_name.as_ptr()))
//         }
//     }

//     fn visit_char_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt32Type()) }
//     }

//     fn visit_def_expr(&mut self, def_expr: DefExpr) -> Self::Return {
//         unimplemented!()
//     }

//     fn visit_f32_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMFloatType()) }
//     }

//     fn visit_f64_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMDoubleType()) }
//     }

//     fn visit_function(&mut self, function: Function) -> Self::Return {
//         let llvm_fn = self.add_or_get_function_profile(function.clone());
//         // check for functions that are just declarations
//         if let None = function.block() {
//             // return the function
//             return LLVMNode::from(llvm_fn);
//         }

//         unsafe {
//             // store the current insert position
//             let llvm_restore_point = LLVMGetInsertBlock(self.builder);
//             LLVMClearInsertionPosition(self.builder);

//             // check if the function has been defined
//             if LLVMCountBasicBlocks(llvm_fn) != 0 {
//                 panic!("duplicate definitinon of function '{}'",
//                        function.identify().name);
//             }
//             // check if the arguments match any other declarations
//             let n_args = LLVMCountParams(llvm_fn) as usize;
//             if n_args != 0 && n_args != function.num_formals() {
//                 panic!("declaration and definition of function '{}' have different arguments",
//                        function.identify().name);
//             }

//             // Build the function definition
//             let mut i = 0;
//             function.for_formals(|formal| {
//                                      let llvm_param = LLVMGetParam(llvm_fn, i as u32);
//                                      let llvm_name = llvm_string(formal.name());
//                                      LLVMSetValueName(llvm_param, llvm_name.as_ptr());
//                                      self.nodes
//                                          .insert(formal.identify(), LLVMNode::from(llvm_param));
//                                      i += 1;
//                                  });

//             // Build the function body definition
//             let llvm_block_name = llvm_string("entry");
//             let block =
//                 LLVMAppendBasicBlockInContext(self.context, llvm_fn, llvm_block_name.as_ptr());
//             LLVMPositionBuilderAtEnd(self.builder, block);

//             // Build the return statement
//             let llvm_definition = self.visit_block_expr(function.block().unwrap());
//             if let LLVMNode::Nil = llvm_definition {
//                 LLVMBuildRetVoid(self.builder);
//             } else {
//                 let llvm_ret = LLVMValueRef::from(llvm_definition);
//                 let llvm_ret_type = LLVMTypeOf(llvm_ret);
//                 if llvm_ret_type == LLVMVoidType() {
//                     LLVMBuildRetVoid(self.builder);
//                 } else {
//                     LLVMBuildRet(self.builder, llvm_ret);
//                 }
//             }

//             // restore the previous insert position
//             LLVMPositionBuilderAtEnd(self.builder, llvm_restore_point);

//             // return the function
//             LLVMNode::from(llvm_fn)
//         }
//     }

//     fn visit_i8_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt8Type()) }
//     }

//     fn visit_i16_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt16Type()) }
//     }

//     fn visit_i32_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt32Type()) }
//     }

//     fn visit_i64_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt64Type()) }
//     }

//     fn visit_item_expr(&mut self, item_expr: ItemExpr) -> Self::Return {
//         match item_expr {
//             ItemExpr::Function(function) => {
//                 self.nodes
//                     .get(&function.identify())
//                     .expect(&format!("use of undeclared function '{}'",
//                                      function.symbolise().name()))
//                     .clone()
//             }
//             ItemExpr::Variable(variable) => {
//                 self.nodes
//                     .get(&variable.identify())
//                     .expect(&format!("use of undeclared variable '{}'",
//                                      variable.symbolise().name()))
//                     .clone()
//             }
//         }
//     }

//     fn visit_lambda_type(&mut self, lambda_type: LambdaType) -> Self::Return {
//         unsafe {
//             LLVMNode::from(LLVMPointerType(
//                 LLVMFunctionType(LLVMTypeRef::from(self.visit_type(lambda_type.typedef())),
//                     lambda_type
//                         .map_formals(|formal| LLVMTypeRef::from(self.visit_type(formal.clone())))
//                         .as_mut_ptr(),
//                     lambda_type.num_formals() as u32,
//                 0), 0
//             ))
//         }
//     }

//     fn visit_literal_expr(&mut self, literal_expr: LiteralExpr) -> Self::Return {
//         unsafe {
//             LLVMNode::from(match literal_expr {
//                                LiteralExpr::Bool(value) => {
//                                    if value {
//                                        LLVMConstInt(LLVMTypeRef::from(self.visit_bool_type()), 1, 0)
//                                    } else {
//                                        LLVMConstInt(LLVMTypeRef::from(self.visit_bool_type()), 0, 0)
//                                    }
//                                }
//                                LiteralExpr::Char(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_u32_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::F32(value) => {
//                                    LLVMConstReal(LLVMTypeRef::from(self.visit_f32_type()),
//                                                  value as f64)
//                                }
//                                LiteralExpr::F64(value) => {
//                                    LLVMConstReal(LLVMTypeRef::from(self.visit_f64_type()),
//                                                  value as f64)
//                                }
//                                LiteralExpr::I8(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_i8_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::I16(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_i16_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::I32(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_i32_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::I64(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_i64_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::Str(value) => {
//                 let string = CString::new(value.clone()).unwrap();
//                 let string_bytes = string.as_bytes_with_nul();
//                 let llvm_string = llvm_string(value.clone());
//                 let llvm_string_global = LLVMBuildGlobalStringPtr(self.builder,
//                                                                   string_bytes.as_ptr() as
//                                                                   *const _,
//                                                                   llvm_string.as_ptr());
//                 LLVMConstGEP(llvm_string_global,
//                              vec![LLVMConstInt(LLVMInt32Type(), 0, 0)].as_mut_ptr(),
//                              1)
//             }
//                                LiteralExpr::U8(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_u8_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::U16(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_u16_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::U32(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_u32_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::U64(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_u64_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                                LiteralExpr::USize(value) => {
//                                    LLVMConstInt(LLVMTypeRef::from(self.visit_usize_type()),
//                                                 value as u64,
//                                                 0)
//                                }
//                            })
//         }
//     }

//     fn visit_module(&mut self, module: Module) -> Self::Return {
//         module.for_functions(|(_, function)| self.add_or_get_function_profile(function.clone()));
//         module.for_functions(|(_, function)| self.visit_function(function.clone()));
//         LLVMNode::Nil
//     }

//     fn visit_process_expr(&mut self, process_expr: ProcessExpr) -> Self::Return {
//         // create a function for the process
//         let process_function = process_expr.as_function();
//         self.visit_function(process_function.clone());
//         // create a function call to the runtime to launch the process function
//         let process_call_expr =
//             CallExpr::new(Identifier::id(),
//                           Expr::from(ItemExpr::Function(self.noir_context().runtime().process_fn())),
//                           vec![Expr::from(ItemExpr::Function(process_function))]);
//         let llvm_call_expr = self.visit_call_expr(process_call_expr);
//         // add the process to the nodes
//         self.nodes
//             .insert(process_expr.identify(),
//                     LLVMNode::from(llvm_call_expr.clone()));
//         llvm_call_expr
//     }

//     fn visit_process_join_expr(&mut self, process_join_expr: ProcessJoinExpr) -> Self::Return {
//         // create a call target from the process join function
//         let llvm_target =
//             LLVMValueRef::from(self.nodes
//                                    .get(&self.noir_context().runtime().process_join_fn().identify())
//                                    .unwrap()
//                                    .clone());
//         // create arguments from the process being joined
//         let mut llvm_arguments =
//             vec![LLVMValueRef::from(self.nodes
//                                         .get(&process_join_expr.process().identify())
//                                         .unwrap()
//                                         .clone())];
//         unsafe {
//             // create the function call
//             let llvm_name = llvm_string("");
//             LLVMNode::from(LLVMBuildCall(self.builder,
//                                          llvm_target,
//                                          llvm_arguments.as_mut_ptr(),
//                                          llvm_arguments.len() as u32,
//                                          llvm_name.as_ptr()))
//         }
//     }

//     fn visit_ptr_type(&mut self, ptr_type: PtrType) -> Self::Return {
//         unsafe {
//             LLVMNode::from(LLVMPointerType(LLVMTypeRef::from(self.visit_type(ptr_type.ty())), 0))
//         }
//     }

//     fn visit_string_type(&mut self) -> Self::Return {
//         self.visit_ptr_type(PtrType::new(Type::I8))
//     }

//     fn visit_struct_expr(&mut self, _: StructExpr) -> Self::Return {
//         unimplemented!()
//     }

//     fn visit_struct_element_expr(&mut self, _: StructElementExpr) -> Self::Return {
//         unimplemented!()
//     }

//     fn visit_struct_type(&mut self, struct_type: StructType) -> Self::Return {
//         unsafe {
//             LLVMNode::from(LLVMStructType(struct_type
//                                .map_elements(|element| {
//                                     LLVMTypeRef::from(self.visit_type(element.typedef()))
//                                 })
//                                .as_mut_ptr(),
//                            struct_type.num_elements() as u32,
//                            0))
//         }
//     }

//     fn visit_u8_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt8Type()) }
//     }

//     fn visit_u16_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt16Type()) }
//     }

//     fn visit_u32_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt32Type()) }
//     }

//     fn visit_u64_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMInt64Type()) }
//     }

//     fn visit_usize_type(&mut self) -> Self::Return {
//         unsafe {
//             let llvm_ptr_size = LLVMPointerSize(LLVMGetModuleDataLayout(self.module));
//             LLVMNode::from(LLVMIntType(llvm_ptr_size))
//         }
//     }

//     fn visit_variable(&mut self, _: Variable) -> Self::Return {
//         unimplemented!()
//     }

//     fn visit_void_type(&mut self) -> Self::Return {
//         unsafe { LLVMNode::from(LLVMVoidType()) }
//     }

//     fn visit_void_expr(&mut self, _: VoidExpr) -> Self::Return {
//         LLVMNode::Nil
//     }
// }

// #[derive(Clone)]
// pub enum LLVMNode {
//     Nil,
//     Value(LLVMValueRef),
//     Type(LLVMTypeRef),
// }

// impl Default for LLVMNode {
//     fn default() -> LLVMNode {
//         LLVMNode::Nil
//     }
// }

// impl From<LLVMTypeRef> for LLVMNode {
//     fn from(ty: LLVMTypeRef) -> LLVMNode {
//         LLVMNode::Type(ty)
//     }
// }

// impl From<LLVMValueRef> for LLVMNode {
//     fn from(value: LLVMValueRef) -> LLVMNode {
//         LLVMNode::Value(value)
//     }
// }

// impl From<LLVMNode> for LLVMTypeRef {
//     fn from(node: LLVMNode) -> LLVMTypeRef {
//         match node {
//             LLVMNode::Nil => panic!("expected LLVMNode::Type got LLVMNode::Nil"),
//             LLVMNode::Type(ty) => ty,
//             LLVMNode::Value(..) => panic!("expected LLVMNode::Type got LLVMNode::Value"),
//         }
//     }
// }

// impl From<LLVMNode> for LLVMValueRef {
//     fn from(node: LLVMNode) -> LLVMValueRef {
//         match node {
//             LLVMNode::Nil => panic!("expected LLVMNode::Value got LLVMNode::Nil"),
//             LLVMNode::Type(..) => panic!("expected LLVMNode::Value got LLVMNode::Type"),
//             LLVMNode::Value(value) => value,
//         }
//     }
// }

// fn llvm_string<S: Into<String>>(string: S) -> CString {
//     CString::new(string.into()).unwrap()
// }

// fn llvm_dump_module(module: LLVMModuleRef) {
//     unsafe {
//         println!("{}",
//                  CStr::from_ptr(LLVMPrintModuleToString(module))
//                      .to_string_lossy()
//                      .into_owned());
//     }
// }

// fn llvm_dump_module_to_string(module: LLVMModuleRef) -> String {
//     unsafe {
//         CStr::from_ptr(LLVMPrintModuleToString(module))
//             .to_string_lossy()
//             .into_owned()
//     }
// }

// fn llvm_dump_node(node: LLVMNode) {
//     match node {
//         LLVMNode::Nil => (),
//         LLVMNode::Type(ty) => llvm_dump_type(ty),
//         LLVMNode::Value(value) => llvm_dump_value(value),
//     }
// }

// fn llvm_dump_type(ty: LLVMTypeRef) {
//     unsafe {
//         println!("{}",
//                  CStr::from_ptr(LLVMPrintTypeToString(ty))
//                      .to_string_lossy()
//                      .into_owned())
//     }
// }

// fn llvm_dump_value(value: LLVMValueRef) {
//     unsafe {
//         println!("{}",
//                  CStr::from_ptr(LLVMPrintValueToString(value))
//                      .to_string_lossy()
//                      .into_owned())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use id::*;
//     use noir::*;

//     #[test]
//     fn main() {
//         // create a noir context
//         let noir_context = Context::new();

//         // an empty main function
//         let main_function = Function::new(Symbol::new("main"),
//                                           Vec::new(),
//                                           Some(BlockExpr::new(
//                                               Identifier::id(), // identifier for this block
//                                               Expr::from(VoidExpr::new( // return expression for this block
//                                                   Identifier::id()
//                                               )),
//                                               Vec::new(), // expressions for this block
//                                               Vec::new() // epilogue for this block
//                                           )),
//                                           LambdaType::new(Vec::new(), Type::Void));

//         // a module containing the main function
//         let mut main_module = Module::new(Symbol::new("main"), Functions::new());
//         main_module.add_function(main_function);

//         // build and dump
//         let mut builder = LLVMBuilder::new(main_module.symbolise().name(), noir_context);
//         builder.visit_module(main_module);
//         builder.build();
//         builder.dump();
//     }

//     #[test]
//     fn writeln() {
//         // create a noir context
//         let noir_context = Context::new();

//         // processes that call writeln
//         let writeln_hello = CallExpr::new(
//             Identifier::id(), // identifier for the function call
//             Expr::from(ItemExpr::Function(noir_context.prelude().writeln_string_fn())), // target for the function call
//             vec![Expr::from(LiteralExpr::Str("hello".to_string()))] // arguments for the function call
//         );
//         let writeln_world = CallExpr::new(
//             Identifier::id(), // identifier for the function call
//             Expr::from(ItemExpr::Function(noir_context.prelude().writeln_string_fn())), // target for the function call
//             vec![Expr::from(LiteralExpr::Str("world".to_string()))] // arguments for the function call
//         );
//         let process_hello = ProcessExpr::new(
//             Identifier::id(), // identifier for the process
//             vec![],
//             vec![],
//             vec![Expr::from(writeln_hello)] // body of expressions for the process
//         );
//         let process_world = ProcessExpr::new(
//             Identifier::id(), // identifier for the process
//             vec![],
//             vec![],
//             vec![Expr::from(writeln_world)] // body of expressions for the process
//         );
//         let join_hello = ProcessJoinExpr::new(Identifier::id(), process_hello.clone());
//         let join_world = ProcessJoinExpr::new(Identifier::id(), process_world.clone());

//         // a main function
//         let main_function_block = Some(BlockExpr::new(Identifier::id(), // identifier for this block
//                                                       Expr::Void(VoidExpr::new(// return expression for this block
//                                                                                Identifier::id())),
//                                                       vec![// expressions for this block
//                                                            Expr::from(process_hello),
//                                                            Expr::from(process_world)],
//                                                       vec![// epilogue for this block
//                                                            Expr::from(join_hello),
//                                                            Expr::from(join_world)]));
//         let main_function = Function::new(Symbol::new("main"),
//                                           Vec::new(),
//                                           main_function_block,
//                                           LambdaType::new(Vec::new(), Type::Void));

//         // a module containing the main function
//         let mut main_module = Module::new(Symbol::new("main"), Functions::new());

//         // build and dump
//         let mut builder = LLVMBuilder::new(main_module.symbolise().name(), noir_context);
//         main_module.add_function(main_function);

//         // visit the module
//         builder.visit_module(main_module);
//         builder.build();
//         builder.dump();
//     }

//     #[test]
//     fn writeln_with_variable() {
//         // create a noir context
//         let noir_context = Context::new();

//         // processes that call writeln
//         let writeln_hello = CallExpr::new(
//             Identifier::id(), // identifier for the function call
//             Expr::from(ItemExpr::Function(noir_context.prelude().writeln_string_fn())), // target for the function call
//             vec![Expr::from(LiteralExpr::Str("hello".to_string()))] // arguments for the function call
//         );
//         let writeln_world = CallExpr::new(
//             Identifier::id(), // identifier for the function call
//             Expr::from(ItemExpr::Function(noir_context.prelude().writeln_string_fn())), // target for the function call
//             vec![Expr::from(LiteralExpr::Str("world".to_string()))] // arguments for the function call
//         );
//         let process_hello = ProcessExpr::new(
//             Identifier::id(), // identifier for the process
//             vec![],
//             vec![],
//             vec![Expr::from(writeln_hello)] // body of expressions for the process
//         );
//         let process_world = ProcessExpr::new(
//             Identifier::id(), // identifier for the process
//             vec![],
//             vec![],
//             vec![Expr::from(writeln_world)] // body of expressions for the process
//         );
//         let join_hello = ProcessJoinExpr::new(Identifier::id(), process_hello.clone());
//         let join_world = ProcessJoinExpr::new(Identifier::id(), process_world.clone());

//         // a main function
//         let main_function_block = Some(BlockExpr::new(Identifier::id(), // identifier for this block
//                                                       Expr::Void(VoidExpr::new(// return expression for this block
//                                                                                Identifier::id())),
//                                                       vec![// expressions for this block
//                                                            Expr::from(process_hello),
//                                                            Expr::from(process_world)],
//                                                       vec![// epilogue for this block
//                                                            Expr::from(join_hello),
//                                                            Expr::from(join_world)]));
//         let main_function = Function::new(Symbol::new("main"),
//                                           Vec::new(),
//                                           main_function_block,
//                                           LambdaType::new(Vec::new(), Type::Void));

//         // a module containing the main function
//         let mut main_module = Module::new(Symbol::new("main"), Functions::new());

//         // build and dump
//         let mut builder = LLVMBuilder::new(main_module.symbolise().name(), noir_context);
//         main_module.add_function(main_function);

//         // visit the module
//         builder.visit_module(main_module);
//         builder.build();
//         builder.dump();
//     }

// }

// use super::ast;
// use super::ast::{IsCompat, Typedef};
// use super::id;
// use super::identifier::{Id, Identify, Name};
// use super::llvm_sys::core::*;
// use super::llvm_sys::prelude::*;

// use std::collections::HashMap;
// use std::ffi::{CStr, CString};
// use std::ptr;

// pub trait Mangle {
//     fn mangle(&self) -> CString;
// }

// impl Mangle for ast::Function {
//     fn mangle(&self) -> CString {
//         self.profile.mangle()
//     }
// }

// impl Mangle for ast::FunctionProfile {
//     fn mangle(&self) -> CString {
//         unimplemented!()
//     }
// }

// impl Mangle for ast::Type {
//     fn mangle(&self) -> CString {
//         unimplemented!()
//     }
// }

// #[derive(Clone)]
// pub enum IRNode {
//     Value(LLVMValueRef),
//     Type(LLVMTypeRef),
//     Nil,
// }

// impl From<LLVMValueRef> for IRNode {
//     fn from(value: LLVMValueRef) -> IRNode {
//         IRNode::Value(value)
//     }
// }

// impl From<LLVMTypeRef> for IRNode {
//     fn from(ty: LLVMTypeRef) -> IRNode {
//         IRNode::Type(ty)
//     }
// }

// impl From<IRNode> for LLVMValueRef {
//     fn from(node: IRNode) -> LLVMValueRef {
//         match node {
//             IRNode::Nil => panic!("error: expected IRNode::Value got IRNode::Nil"),
//             IRNode::Type(_) => panic!("error: expected IRNode::Value got IRNode::Type"),
//             IRNode::Value(value) => value,
//         }
//     }
// }

// impl From<IRNode> for LLVMTypeRef {
//     fn from(node: IRNode) -> LLVMTypeRef {
//         match node {
//             IRNode::Nil => panic!("error: expected IRNode::Type got IRNode::Nil"),
//             IRNode::Type(ty) => ty,
//             IRNode::Value(_) => panic!("error: expected IRNode::Type got IRNode::Value"),
//         }
//     }
// }

// pub fn codegen(root: &ast::Node, core_modules: &Vec<Box<ast::Module>>) -> String {
//     let mut builder = IRBuilder::new(root.name().as_str());
//     builder.codegen(root, core_modules)
// }

// pub struct IRBuilder {
//     pub context: LLVMContextRef,
//     pub module: LLVMModuleRef,
//     pub builder: LLVMBuilderRef,
//     pub nodes: HashMap<id::Identifier, IRNode>,
//     pub addresses: HashMap<id::Identifier, IRNode>,
// }

// impl IRBuilder {
//     pub fn new(name: &str) -> IRBuilder {
//         unsafe {
//             let context = LLVMContextCreate();
//             let module = LLVMModuleCreateWithName(name.as_ptr() as *const _);
//             let builder = LLVMCreateBuilderInContext(context);
//             IRBuilder {
//                 context: context,
//                 module: module,
//                 builder: builder,
//                 nodes: HashMap::new(),
//                 addresses: HashMap::new(),
//             }
//         }
//     }

//     pub fn codegen(&mut self, root: &ast::Node, core_modules: &Vec<Box<ast::Module>>) -> String {
//         self.codegen_function_profile(&ast::ASYNC_FN);
//         self.codegen_function_profile(&ast::AWAIT_FN);
//         self.codegen_function_profile(&ast::JOIN_FN);
//         for core_module in core_modules {
//             self.codegen_module(core_module);
//         }
//         self.codegen_node(root);
//         unsafe {
//             CStr::from_ptr(LLVMPrintModuleToString(self.module))
//                 .to_string_lossy()
//                 .into_owned()
//         }
//     }

//     pub fn codegen_node(&mut self, node: &ast::Node) -> IRNode {
//         match *node {
//             ast::Node::Expr(ref expr) => self.codegen_expr(expr),
//             ast::Node::Item(ref item) => self.codegen_item(item),
//             ast::Node::Nil => IRNode::Nil,
//         }
//     }

//     pub fn codegen_expr(&mut self, expr: &ast::Expr) -> IRNode {
//         match *expr {
//             ast::Expr::Assign(ref expr) => self.codegen_assign_expr(expr),
//             ast::Expr::Async(ref expr) => self.codegen_async_expr(expr),
//             ast::Expr::Await(ref expr) => self.codegen_await_expr(expr),
//             ast::Expr::Block(ref expr) => self.codegen_block_expr(expr),
//             ast::Expr::Call(ref expr) => self.codegen_call_expr(expr),
//             ast::Expr::Ident(ref expr) => self.codegen_ident_expr(expr),
//             ast::Expr::IdentPath(ref expr) => self.codegen_ident_path_expr(expr),
//             ast::Expr::Join(ref expr) => self.codegen_join_expr(expr),
//             ast::Expr::Literal(ref expr) => self.codegen_literal_expr(expr),
//             ast::Expr::Struct(ref expr) => self.codegen_struct_expr(expr),
//             ast::Expr::Void(ref expr) => self.codegen_void_expr(expr),
//         }
//     }

//     pub fn codegen_item(&mut self, item: &ast::Item) -> IRNode {
//         match *item {
//             ast::Item::Function(ref item) => self.codegen_function(item),
//             ast::Item::FunctionProfile(ref item) => self.codegen_function_profile(item),
//             ast::Item::Module(ref item) => self.codegen_module(item),
//             ast::Item::Type(ref item) => self.codegen_type(item),
//             ast::Item::Unresolved(ref item) => self.codegen_unresolved(item),
//             ast::Item::Variable(ref item) => self.codegen_variable(item),
//         }
//     }

//     pub fn codegen_assign_expr(&mut self, assign_expr: &ast::AssignExpr) -> IRNode {
//         self.codegen_expr(assign_expr.target.as_ref());
//         let target_address = self.addresses.get(assign_expr.target.identify()).unwrap().clone();
//         unsafe {
//             let node = LLVMBuildStore(self.builder,
//                            LLVMValueRef::from(self.codegen_expr(assign_expr.definition.as_ref())),
//                            LLVMValueRef::from(target_address),);
//             IRNode::from(node)
//         }
//     }

//     pub fn codegen_async_expr(&mut self, async_expr: &ast::AsyncExpr) -> IRNode {
//         if let Some(node) = self.nodes.get(async_expr.identify()) {
//             return node.clone();
//         }
//         self.codegen_function(async_expr.function.as_ref());
//         let node = self.codegen_call_expr(&ast::CallExpr::new(
//             async_expr.identify().clone(),
//             Box::new(ast::Expr::Ident(ast::IdentExpr::new(
//                 id::Identifier::id(),
//                 Box::new(ast::Item::FunctionProfile(ast::ASYNC_FN.clone()))
//             ))),
//             vec![
//                 Box::new(ast::Expr::Ident(ast::IdentExpr::new(
//                     id::Identifier::id(),
//                     Box::new(ast::Item::Function(*async_expr.function.clone()))
//                 ))),
//                 Box::new(ast::Expr::Struct(*async_expr.env.clone()))
//             ]
//         ));
//         self.nodes
//             .insert(async_expr.identify().clone(), node.clone());
//         node
//     }

//     pub fn codegen_await_expr(&mut self, await_expr: &ast::AwaitExpr) -> IRNode {
//         if let Some(node) = self.nodes.get(await_expr.identify()) {
//             return node.clone();
//         }

//         let env_ty = await_expr.typedef();
//         let env_ty = if let ast::Type::Async(async_ty) = env_ty {
//             ast::RefType::new(Box::new(ast::Type::Struct(ast::StructType::new(
//                 id::Identifier::id(),
//                 vec![Box::new(ast::Variable::new(id::Identifier::id(), async_ty.inner_ty, true, true))],
//                 false
//             ))) , false)
//         } else {
//             ast::RefType::new(Box::new(ast::Type::Struct(ast::StructType::new(
//                 id::Identifier::id(),
//                 vec![Box::new(ast::Variable::new(id::Identifier::id(), Box::new(env_ty), true, true))],
//                 false
//             ))) , false)
//         };

//         unsafe {
//             let llvm_name = llvm_string("");
//             let await_node = LLVMBuildCall(self.builder,
//                                            LLVMValueRef::from(self.nodes
//                                                                   .get(ast::AWAIT_FN
//                                                                            .identify())
//                                                                   .unwrap()
//                                                                   .clone()),
//                                            vec![
//                     LLVMValueRef::from(self.codegen_expr(await_expr.inner_expr.as_ref()))
//                 ]
//                                                    .as_mut_ptr(),
//                                            1u32,
//                                            llvm_name.as_ptr());

//             let llvm_result = LLVMBuildBitCast(self.builder,
//                                                LLVMValueRef::from(await_node),
//                                                LLVMTypeRef::from(self.codegen_ref_type(&env_ty)),
//                                                llvm_name.as_ptr());
//             let llvm_name = llvm_string("");
//             let llvm_result = LLVMBuildStructGEP(self.builder,
//                                                  LLVMValueRef::from(llvm_result),
//                                                  0 as u32, // the result is always stored at the first index
//                                                  llvm_name.as_ptr());
//             let node = IRNode::from(LLVMBuildLoad(self.builder, llvm_result, llvm_name.as_ptr()));
//             self.nodes
//                 .insert(await_expr.identify().clone(), node.clone());
//             node
//         }
//     }

//     pub fn codegen_block_expr(&mut self, block_expr: &ast::BlockExpr) -> IRNode {
//         for expr in block_expr.prelude.iter() {
//             self.codegen_expr(expr.as_ref());
//         }
//         for expr in block_expr.block.iter() {
//             self.codegen_expr(expr.as_ref());
//         }
//         for expr in block_expr.epilogue.iter() {
//             self.codegen_expr(expr.as_ref());
//         }
//         self.codegen_expr(block_expr.definition.as_ref())
//     }

//     pub fn codegen_call_expr(&mut self, call_expr: &ast::CallExpr) -> IRNode {
//         unsafe {
//             let target = LLVMValueRef::from(self.codegen_expr(call_expr.target.as_ref()));
//             let formals = if let ast::Type::Lambda(lambda_ty) = call_expr.target.typedef() {
//                 lambda_ty.formal_tys.clone()
//             } else {
//                 panic!("internal error: call target must be a function")
//             };
//             let mut args = call_expr
//                 .arguments
//                 .iter()
//                 .zip(formals.iter())
//                 .map(|(arg, formal)| {
//                     let llvm_name = llvm_string("arg");
//                     if arg.typedef().is_compat(&formal.typedef()) {
//                         LLVMValueRef::from(self.codegen_expr(arg.as_ref()))
//                     } else {
//                         LLVMBuildBitCast(self.builder,
//                                          LLVMValueRef::from(self.codegen_expr(arg.as_ref())),
//                                          LLVMTypeRef::from(self.codegen_type(formal.as_ref())),
//                                          llvm_name.as_ptr())
//                     }
//                 })
//                 .collect::<Vec<_>>();
//             let node = IRNode::from(LLVMBuildCall(self.builder,
//                                                   target,
//                                                   args.as_mut_ptr(),
//                                                   args.len() as u32,
//                                                   CString::new("").unwrap().as_ptr()));
//             node
//         }
//     }

//     pub fn codegen_function(&mut self, function: &ast::Function) -> IRNode {
//         self.codegen_function_profile(function.profile.as_ref());

//         unsafe {
//             let llvm_restore_point = LLVMGetInsertBlock(self.builder);
//             LLVMClearInsertionPosition(self.builder);

//             // Get llvm function
//             let llvm_name = CString::new(if function.profile.is_extern ||
//                                             function.identify().name == "main" {
//                                              format!("{}", function.name())
//                                          } else {
//                                              format!("{}_{}", function.name(), function.id())
//                                          })
//                     .unwrap();
//             let llvm_fn = LLVMGetNamedFunction(self.module, llvm_name.as_ptr());
//             if llvm_fn == ptr::null_mut() {
//                 panic!("error: use of undefined function '{}'",
//                        function.identify().name);
//             }
//             // Check if the function has been defined
//             if LLVMCountBasicBlocks(llvm_fn) != 0 {
//                 panic!("error: redeclaration of function '{}'",
//                        function.identify().name);
//             }
//             // Check if the arguments match any other declarations
//             let n_args = LLVMCountParams(llvm_fn) as usize;
//             if n_args != 0 && n_args != function.profile.formals.len() {
//                 panic!("error: redeclaration of function '{}' with different arguments",
//                        function.identify().name);
//             }

//             // Build the function definition
//             for (i, formal) in function.profile.formals.iter().enumerate() {
//                 let llvm_param = LLVMGetParam(llvm_fn, i as u32);
//                 let llvm_name = CString::new(formal.identify().name.clone()).unwrap();
//                 LLVMSetValueName(llvm_param, llvm_name.as_ptr());
//                 self.nodes
//                     .insert(formal.identify().clone(), IRNode::from(llvm_param));
//             }

//             // Build the function body definition
//             let llvm_entry = llvm_string("entry");
//             let block = LLVMAppendBasicBlockInContext(self.context, llvm_fn, llvm_entry.as_ptr());
//             LLVMPositionBuilderAtEnd(self.builder, block);

//             // Build the return statement
//             let llvm_definition = self.codegen_expr(function.definition.as_ref());
//             if let IRNode::Nil = llvm_definition {
//                 LLVMBuildRetVoid(self.builder);
//             } else {
//                 let llvm_ret = LLVMValueRef::from(llvm_definition);
//                 let llvm_ret_type = LLVMTypeOf(llvm_ret);
//                 if llvm_ret_type == LLVMVoidType() {
//                     LLVMBuildRetVoid(self.builder);
//                 } else {
//                     LLVMBuildRet(self.builder, llvm_ret);
//                 }
//             }

//             LLVMPositionBuilderAtEnd(self.builder, llvm_restore_point);
//             IRNode::from(llvm_fn)
//         }
//     }

//     pub fn codegen_function_profile(&mut self, function_profile: &ast::FunctionProfile) -> IRNode {
//         unsafe {
//             // Check if function exists
//             let llvm_name =
//                 CString::new(if function_profile.is_extern || function_profile.name() == "main" {
//                                  format!("{}", function_profile.name())
//                              } else {
//                                  format!("{}_{}", function_profile.name(), function_profile.id())
//                              })
//                         .unwrap();
//             if LLVMGetNamedFunction(self.module, llvm_name.as_ptr()) != ptr::null_mut() {
//                 panic!("error: duplicate declaration of {}",
//                        function_profile.name());
//             }
//             // Build the function prototype
//             let llvm_fn = LLVMFunctionType(LLVMTypeRef::from(self.codegen_type(function_profile.ty.as_ref())),
//                              function_profile
//                                  .formals
//                                  .iter()
//                                  .map(|formal| LLVMTypeRef::from(self.codegen_type(formal.ty.as_ref())))
//                                  .collect::<Vec<_>>()
//                                  .as_mut_ptr(),
//                              function_profile.formals.len() as u32,
//                              0);
//             let llvm_fn = LLVMAddFunction(self.module, llvm_name.as_ptr(), llvm_fn);
//             self.nodes
//                 .insert(function_profile.identify().clone(), IRNode::from(llvm_fn));
//             IRNode::from(llvm_fn)
//         }
//     }

//     pub fn codegen_ident_expr(&mut self, ident_expr: &ast::IdentExpr) -> IRNode {
//         match self.nodes.get(ident_expr.item.identify()) {
//             Some(node) => (*node).clone(),
//             _ => {
//                 panic!("error: use of undefined identifier '{}'",
//                        ident_expr.item.name())
//             }
//         }
//     }

//     pub fn codegen_ident_path_expr(&mut self, ident_path_expr: &ast::IdentPathExpr) -> IRNode {
//         let target = self.codegen_expr(ident_path_expr.target.as_ref());
//         if let ast::Type::Ref(ref_ty) = ident_path_expr.target.typedef() {
//             if let ast::Type::Struct(struct_ty) = *ref_ty.ty {

//                 if let ast::Expr::Ident(ref ident_expr) = *ident_path_expr.inner.as_ref() {
//                     if let ast::Item::Variable(ref variable) = *ident_expr.item.as_ref() {
//                         let mut field_index_counter: i64 = -1;
//                         let mut field_index: i64 = -1;
//                         for struct_field in struct_ty.struct_fields.iter() {
//                             field_index_counter += 1;
//                             if struct_field.name() == variable.name() {
//                                 field_index = field_index_counter;
//                             }
//                         }
//                         if field_index < 0 {
//                             panic!("internal error: could not find struct element '{}'",
//                                    variable.name());
//                         }
//                         unsafe {
//                             let llvm_name = llvm_string(variable.name().clone());
//                             let llvm_elem_ptr = LLVMBuildStructGEP(self.builder,
//                                                                    LLVMValueRef::from(target),
//                                                                    field_index as u32,
//                                                                    llvm_name.as_ptr());
//                             self.addresses.insert(ident_path_expr.identify().clone(), IRNode::from(llvm_elem_ptr.clone()));
//                             let node = IRNode::from(LLVMBuildLoad(self.builder,
//                                                                   llvm_elem_ptr,
//                                                                   llvm_name.as_ptr()));

//                             node
//                         }
//                     } else {
//                         IRNode::Nil
//                     }
//                 } else {
//                     IRNode::Nil
//                 }
//             } else {
//                 IRNode::Nil
//             }
//         } else {
//             IRNode::Nil
//         }
//     }

//     pub fn codegen_join_expr(&mut self, join_expr: &ast::JoinExpr) -> IRNode {
//         unsafe {
//             // Join the thread
//             IRNode::from(LLVMBuildCall(self.builder,
//                                        LLVMValueRef::from(self.nodes
//                                                               .get(ast::JOIN_FN.identify())
//                                                               .unwrap()
//                                                               .clone()),
//                                        vec![LLVMValueRef::from(self.nodes
//                                                                    .get(join_expr
//                                                                             .inner_expr
//                                                                             .identify())
//                                                                    .unwrap()
//                                                                    .clone())]
//                                                .as_mut_ptr(),
//                                        1u32,
//                                        CString::new("").unwrap().as_ptr()))
//         }
//     }

//     pub fn codegen_lambda_type(&mut self, lambda_type: &ast::LambdaType) -> IRNode {
//         unsafe {
//             IRNode::from(LLVMPointerType(LLVMFunctionType(LLVMTypeRef::from(self.codegen_type(lambda_type.ty.as_ref())),
//                                              lambda_type
//                                                  .formal_tys
//                                                  .iter()
//                                                  .map(|formal_ty| {
//                                                           LLVMTypeRef::from(self.codegen_type(formal_ty))
//                                                       })
//                                                  .collect::<Vec<_>>()
//                                                  .as_mut_ptr(),
//                                              lambda_type.formal_tys.len() as u32,
//                                              0),
//                             0))
//         }
//     }

//     pub fn codegen_literal_expr(&mut self, literal_expr: &ast::LiteralExpr) -> IRNode {
//         unsafe {
//             IRNode::from(match *literal_expr {
//                              ast::LiteralExpr::Bool(_, value) => {
//                                  LLVMConstInt(LLVMTypeRef::from(self.codegen_type(&ast::Type::Bool)), value as u64, 0)
//                              }
//                              ast::LiteralExpr::Float(_, value) => {
//                                  LLVMConstReal(LLVMTypeRef::from(self.codegen_type(&ast::Type::F64)), value as f64)
//                              }
//                              ast::LiteralExpr::Int(_, value) => {
//                                  LLVMConstInt(LLVMTypeRef::from(self.codegen_type(&ast::Type::I64)),
//                                               value as u64,
//                                               0)
//                              }
//                          })
//         }
//     }

//     pub fn codegen_module(&mut self, module: &ast::Module) -> IRNode {
//         // Codegen module declarations
//         for item in module.items.iter() {
//             match *item.as_ref() {
//                 ast::Item::Module(ref submodule) => {
//                     self.codegen_module(submodule);
//                 }
//                 _ => (),
//             }
//         }

//         // Codegen type declarations
//         for item in module.items.iter() {
//             match *item.as_ref() {
//                 ast::Item::Type(ref ty) => {
//                     self.codegen_type(ty);
//                 }
//                 _ => (),
//             }
//         }

//         // Codegen function profiles
//         for item in module.items.iter() {
//             match *item.as_ref() {
//                 ast::Item::Function(ref function) => {
//                     self.codegen_function(function);
//                 }
//                 ast::Item::FunctionProfile(ref function_profile) => {
//                     self.codegen_function_profile(function_profile);
//                 }
//                 _ => (),
//             }
//         }

//         IRNode::Nil
//     }

//     pub fn codegen_ref_type(&mut self, ref_type: &ast::RefType) -> IRNode {
//         unsafe { IRNode::from(LLVMPointerType(LLVMTypeRef::from(self.codegen_type(ref_type.ty.as_ref())), 0)) }
//     }

//     pub fn codegen_struct_expr(&mut self, struct_expr: &ast::StructExpr) -> IRNode {
//         unsafe {
//             let llvm_name = llvm_string("tmp");
//             let llvm_struct_expr =
//                 LLVMBuildAlloca(self.builder,
//                                 LLVMTypeRef::from(self.codegen_struct_type(struct_expr
//                                                                                .struct_ty
//                                                                                .as_ref())),
//                                 llvm_name.as_ptr());
//             let mut field_index = 0;
//             for struct_field_expr in struct_expr.struct_field_exprs.iter() {
//                 if let ast::Expr::Void(..) = *struct_field_expr.as_ref() {
//                     field_index += 1;
//                     continue;
//                 }
//                 let llvm_name = llvm_string("tmp");
//                 let llvm_field = LLVMBuildStructGEP(self.builder,
//                                                     LLVMValueRef::from(llvm_struct_expr),
//                                                     field_index as u32,
//                                                     llvm_name.as_ptr());
//                 LLVMBuildStore(self.builder,
//                                LLVMValueRef::from(self.codegen_expr(struct_field_expr.as_ref())),
//                                llvm_field);
//                 field_index += 1;
//             }
//             IRNode::from(llvm_struct_expr)
//         }
//     }

//     pub fn codegen_struct_type(&mut self, struct_type: &ast::StructType) -> IRNode {
//         unsafe {
//             IRNode::from(LLVMStructType(struct_type
//                                .struct_fields
//                                .iter()
//                                .map(|struct_field| {
//                                         LLVMTypeRef::from(self.codegen_type(struct_field.ty.as_ref()))
//                                     })
//                                .collect::<Vec<_>>()
//                                .as_mut_ptr(),
//                            struct_type.struct_fields.len() as u32,
//                            0))
//         }
//     }

//     pub fn codegen_type(&mut self, ty: &ast::Type) -> IRNode {
//         unsafe {
//             match *ty {
//                 ast::Type::Async(..) => {
//                     self.codegen_ref_type(&ast::RefType::new(Box::new(ast::Type::I8), false))
//                 }
//                 ast::Type::Unresolved(..) => IRNode::Nil,
//                 ast::Type::Bool => IRNode::from(LLVMInt1Type()),
//                 ast::Type::F32 => IRNode::from(LLVMFloatType()),
//                 ast::Type::F64 => IRNode::from(LLVMDoubleType()),
//                 ast::Type::I8 => IRNode::from(LLVMInt8Type()),
//                 ast::Type::I16 => IRNode::from(LLVMInt16Type()),
//                 ast::Type::I32 => IRNode::from(LLVMInt32Type()),
//                 ast::Type::I64 => IRNode::from(LLVMInt64Type()),
//                 ast::Type::Lambda(ref ty) => self.codegen_lambda_type(ty),
//                 ast::Type::Ref(ref ty) => self.codegen_ref_type(ty),
//                 ast::Type::Struct(ref ty) => self.codegen_struct_type(ty),
//                 ast::Type::Void => IRNode::from(LLVMVoidType()),
//             }
//         }
//     }

//     pub fn codegen_unresolved(&mut self, _: &ast::Unresolved) -> IRNode {
//         unimplemented!()
//     }

//     pub fn codegen_variable(&mut self, _: &ast::Variable) -> IRNode {
//         unimplemented!()
//     }

//     pub fn codegen_void_expr(&mut self, _: &ast::VoidExpr) -> IRNode {
//         IRNode::Nil
//     }
// }

// impl Drop for IRBuilder {
//     fn drop(&mut self) {
//         unsafe {
//             LLVMDisposeBuilder(self.builder);
//             LLVMDisposeModule(self.module);
//             LLVMContextDispose(self.context);
//         }
//     }
// }

// fn llvm_string<T: Into<String>>(string: T) -> CString {
//     CString::new(string.into()).unwrap()
// }

// fn llvm_dump_module(module: LLVMModuleRef) {
//     unsafe {
//         println!("{}",
//                  CStr::from_ptr(LLVMPrintModuleToString(module))
//                      .to_string_lossy()
//                      .into_owned());
//     }
// }

// fn llvm_dump_node(node: IRNode) {
//     unsafe {
//         match node {
//             IRNode::Nil => (),
//             IRNode::Type(ty) => {
//                 println!("{}",
//                          CStr::from_ptr(LLVMPrintTypeToString(ty))
//                              .to_string_lossy()
//                              .into_owned())
//             }
//             IRNode::Value(value) => {
//                 println!("{}",
//                          CStr::from_ptr(LLVMPrintValueToString(value))
//                              .to_string_lossy()
//                              .into_owned())
//             }
//         }

//     }
// }