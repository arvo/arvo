use super::*;

///
#[derive(Clone)]
pub struct Context {
    pub function_table: FunctionTable,
    pub module_table: ModuleTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Context {
        Context {
            function_table: FunctionTable::new(),
            module_table: ModuleTable::new(),
            type_table: TypeTable::new(),
        }
    }
}