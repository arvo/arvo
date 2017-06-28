#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate regex;

// The base IR is declared first so that other IRs can use its macros.
#[macro_use]
pub mod bair;

// The IRs are declared second so that they can use the base 
#[macro_use]
pub mod air;
// #[macro_use]
// pub mod ast;
#[macro_use]
pub mod noir;

// Compiler passes are exported so that other libraries and binaries can use
// them.
// pub mod builder;
pub mod identifier;
pub mod normaliser;
// pub mod parser;
// pub mod resolver;
