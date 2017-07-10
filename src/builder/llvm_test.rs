use super::llvm::*;
use super::super::identifier::Symbol;
use super::super::noir::*;

#[test]
fn hello_world() {
    let context = context::Context::new();
    let mut builder = LLVMBuilder::new("test", context.clone());

    let process_hello = ProcessExpr::new(
        BlockExpr::new(
            Exprs::new(), // prelude
            Exprs::new(), // epilogue
            vec![ 
                CallExpr::new(
                    ItemExpr::new(
                        context.prelude.writeln_string_fn.clone()
                    ), // target
                    vec![
                        LiteralExpr::new(
                            Literal::Str("hello".to_string())
                        )
                    ], // arguments
                )
            ], // body
            VoidExpr::new(), // ret
            FunctionTable::new(), // functions
            ModuleTable::new(), // modules
            TypeTable::new(), // types
        )
    );
    let process_world = ProcessExpr::new(
        BlockExpr::new(
            Exprs::new(), // prelude
            Exprs::new(), // epilogue
            vec![ 
                CallExpr::new(
                    ItemExpr::new(
                        context.prelude.writeln_string_fn.clone()
                    ), // target
                    vec![
                        LiteralExpr::new(
                            Literal::Str("world".to_string())
                        )
                    ], // arguments
                )
            ], // body
            VoidExpr::new(), // ret
            FunctionTable::new(), // functions
            ModuleTable::new(), // modules
            TypeTable::new(), // types
        )
    );
    let process_join_hello = ProcessJoinExpr::new(process_hello.clone());
    let process_join_world = ProcessJoinExpr::new(process_world.clone());

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

    builder.codegen_runtime();
    builder.codegen_prelude();
    builder.codegen_function(&main_fn);

    builder.dump();
    builder.build();
}