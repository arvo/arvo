//! Resolver

use super::air::*;
use super::ast;
use super::identifier::Symbolise;

pub struct Resolver {
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {}
    }

    pub fn resolve_module(&mut self, module: ast::Module) -> Module {
        Module::new(module.symbolise(), Functions::new(), Modules::new(), Types::new())
    }
}