use super::air;
use super::identifier::Identify;
use super::noir::*;

pub struct Normaliser {
}

impl Normaliser {
    pub fn new() -> Normaliser {
        Normaliser {}
    }

    pub fn normalise_void_expr(&mut self, void_expr: air::VoidExpr) -> VoidExpr {
        VoidExpr::new(void_expr.identify())
    }
}