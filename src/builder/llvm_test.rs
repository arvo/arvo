use super::llvm::*;
use super::super::identifier::Symbol;
use super::super::noir::*;
use super::super::noir::context::*;

#[test]
fn hello_world() {
    let context = Context::new();
    let mut builder = LLVMBuilder::new("test", context.clone());

    let (process_hello, process_join_hello) = simple_process(vec![writeln_call(&context, "hello")]);
    let (process_world, process_join_world) = simple_process(vec![writeln_call(&context, "world")]);

    let main_fn = Function::new(
        Symbol::new("main"),
        Variables::new(),
        PrimitiveType::Void,
        BlockExpr::new(
            Exprs::new(), // prelude
            vec![
                process_join_hello,
                process_join_world,
            ], // epilogue
            vec![ 
                process_hello,
                process_world,
            ], // body
            VoidExpr::new(), // ret
            FunctionTable::new(), // functions
            ModuleTable::new(), // modules
            TypeTable::new(), // types
        )
    );

    // make sure the runtime and the prelude are present
    builder.codegen_runtime();
    builder.codegen_prelude();
    builder.codegen_function(&main_fn);

    // dump the IR and
    builder.dump();
    builder.build();
}

fn simple_process<E: Into<Expr>>(body: Vec<E>) -> (ProcessExpr, ProcessJoinExpr) {
    let process = ProcessExpr::new(BlockExpr::new(
            Exprs::new(), // prelude
            Exprs::new(), // epilogue
            body, // body
            VoidExpr::new(), // ret
            FunctionTable::new(), // functions
            ModuleTable::new(), // modules
            TypeTable::new(), // types
        ));
    let process_join = ProcessJoinExpr::new(process.clone());
    (process, process_join)
}

fn writeln_call<X: Into<String>>(context: &Context, x: X) -> CallExpr {
    CallExpr::new(
        ItemExpr::new(
            context.prelude.writeln_string_fn.clone()
        ), // target
        vec![
            LiteralExpr::new(
                Literal::Str(x.into())
            )
        ], // arguments
    )
}