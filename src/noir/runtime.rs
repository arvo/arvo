use super::*;

pub struct Runtime {
    process_fn: Function,
    process_join_fn: Function,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            process_fn: Function::new(// symbol of the function
                                      Symbol::new("__libruntime__process"),
                                      // formals of the function
                                      vec![Variable::new(Symbol::new("f"),
                                                         LambdaType::new(vec![],
                                                                         PrimitiveType::Void
                                                                             .into())
                                                                 .into())],
                                      // type profile of the function
                                      PtrType::new(PrimitiveType::I8.into()).into(),
                                      // definition of the function
                                      None),
            process_join_fn: Function::new(// symbol of the function
                                           Symbol::new("__libruntime__process_join"),
                                           // formals of the function
                                           vec![Variable::new(Symbol::new("process"),
                                                              PtrType::new(PrimitiveType::I8
                                                                               .into())
                                                                      .into())],
                                           // type profile of the function
                                           PrimitiveType::Void.into(),
                                           // definition of the function
                                           None),
        }
    }

    pub fn process_fn(&self) -> Function {
        self.process_fn.clone()
    }

    pub fn process_join_fn(&self) -> Function {
        self.process_join_fn.clone()
    }
}