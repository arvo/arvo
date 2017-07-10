use super::*;

#[derive(Clone)]
pub struct Runtime {
    pub process_fn: Function,
    pub process_join_fn: Function,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            process_fn: Function::new(
                Symbol::new("__libruntime__process"),
                vec![
                    Variable::new(Symbol::new("f"), LambdaType::new(Types::new(), PrimitiveType::Void))
                ],
                PtrType::new(PrimitiveType::I8),
                None
            ),
            process_join_fn: Function::new(
                Symbol::new("__libruntime__process_join"),
                vec![
                    Variable::new(Symbol::new("process"), PtrType::new(PrimitiveType::I8))
                ],
                PrimitiveType::Void,
                None
            ),
        }
    }

    pub fn functions(&self) -> Functions {
        vec![self.process_fn.clone(),
             self.process_join_fn.clone()]
    }
}