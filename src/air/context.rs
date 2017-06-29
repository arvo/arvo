use super::prelude::Prelude;

pub struct Context {
    prelude: Prelude,
}

impl Context {
    pub fn new() -> Context {
        Context {
            prelude: Prelude::new(),
        }
    }

    pub fn prelude(&self) -> &Prelude {
        &self.prelude
    }
}