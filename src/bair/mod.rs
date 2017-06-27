//! Base Intermediate Representation
//!
//! The base intermediate representation (BaIR) is a collection of IR 
//! definitions that are common throughout all other IRs.

#[macro_use]
pub mod macros;

/// An `Object` is a wrapper type that uses an `Arc` to manage ownership of an
/// `RwLock` that provides thread-safe access to a `RefCell` that exposes
/// mutability.
pub type Object<T> = ::std::sync::Arc<::std::sync::RwLock<::std::cell::RefCell<T>>>;

/// A `PrimitiveType` is one that is native to the language. It does not
/// require any definitions, or any modules, to be understood by the compiler.
#[derive(Clone)]
pub enum PrimitiveType {
    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    Str,
    U8,
    U16,
    U32,
    U64,
    USize,
    Void,
}