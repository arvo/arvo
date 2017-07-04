use super::*;
use super::prelude::Prelude;
use super::runtime::Runtime;

pub struct Context {
    pub prelude: Prelude,
    pub runtime: Runtime,
    pub function_table: FunctionTable,
    pub module_table: ModuleTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Context {
        Context {
            prelude: Prelude::new(),
            runtime: Runtime::new(),
            function_table: FunctionTable::new(),
            module_table: ModuleTable::new(),
            type_table: TypeTable::new(),
        }
    }
}