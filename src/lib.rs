#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate regex;

pub mod identifier;
pub mod lexer;
pub mod parser;
pub mod ast;

pub mod compile;
pub use compile::compile;

pub mod repl;
pub use repl::repl;