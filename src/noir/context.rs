use super::prelude::Prelude;
use super::runtime::Runtime;

pub struct Context {
    prelude: Prelude,
    runtime: Runtime,
}

impl Context {
    pub fn new() -> Context {
        Context {
            prelude: Prelude::new(),
            runtime: Runtime::new(),
        }
    }

    pub fn prelude(&self) -> &Prelude {
        &self.prelude
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
}