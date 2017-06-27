#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate regex;

/// Intermediate representations (IRs) are abstract data types that are used
/// to represent a program at different stages of its compilation.

/// The base IR is declared first so that other IRs can use its macros.
#[macro_use]
pub mod bair;

/// The IRs are declared second so that they can use the base IR macros and
/// other modules can use the IR macros.
#[macro_use]
pub mod air;
#[macro_use]
pub mod ast;
#[macro_use]
pub mod noir;

pub mod builder;
pub mod identifier;
pub mod normaliser;
pub mod parser;
pub mod resolver;
