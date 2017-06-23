use super::*;

pub struct Runtime {
    process_fn: Function,
    process_join_fn: Function,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            process_fn:
                Function::new(// symbol of the function
                              Symbol::new("__libruntime__process"),
                              // formals of the function
                              vec![Variable::new(Symbol::new("f"),
                                                 Type::Lambda(LambdaType::new(Types::new(),
                                                                              Type::Primitive(PrimitiveType::Void))))],
                              // definition of the function
                              None,
                              // type profile of the function
                              LambdaType::new(vec![Type::Lambda(LambdaType::new(Types::new(),
                                                                                Type::Primitive(PrimitiveType::Void)))],
                                              Type::Ptr(PtrType::new(Type::Primitive(PrimitiveType::I8))))),
            process_join_fn:
                Function::new(// symbol of the function
                              Symbol::new("__libruntime__process_join"),
                              // formals of the function
                              vec![Variable::new(Symbol::new("process"),
                                                 Type::Ptr(PtrType::new(Type::Primitive(PrimitiveType::I8))))],
                              // definition of the function
                              None,
                              // type profile of the function
                              LambdaType::new(vec![Type::Ptr(PtrType::new(Type::Primitive(PrimitiveType::I8)))], Type::Primitive(PrimitiveType::Void))),
        }
    }

    pub fn process_fn(&self) -> Function {
        self.process_fn.clone()
    }

    pub fn process_join_fn(&self) -> Function {
        self.process_join_fn.clone()
    }
}