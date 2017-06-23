/// Create a new `Object`. This needs to be a macro because `Object` is not an
/// alias type, as such it cannot have methods.
#[macro_export]
macro_rules! object {
    ($inner: expr) => (::std::sync::Arc::new(::std::sync::RwLock::new(::std::cell::RefCell::new($inner))))
}

/// Send a message to an `Object`. An immutable message cannot result in the 
/// modification of the `Object`. Using the `loop` suffix will iterate over
/// iterand returned by the message.
#[macro_export]
macro_rules! object_proxy {
    ($object: expr => $($method: ident ( $($arg: expr ),* )).+) => {{
        let inner = $object.read().unwrap();
        let inner = inner.borrow();
        inner.$($method($($arg),*)).+
    }};
    ($object: expr => $($method: ident ( $($arg: expr ),* )).+ loop $item: ident {
        $($expr: expr ;)*
    }) => {{
        let inner = $object.read().unwrap();
        let inner = inner.borrow();
        for $item in inner.$($method($($arg),*)).+ {
            $($expr ;)*
        }
    }};
}

/// Send a message to an `Object`. Using the `mut` prefix will allow the
/// message to modify the target. Using the `loop` suffix will iterate over
/// iterand returned by the message.
#[macro_export]
macro_rules! object_proxy_mut {
    ($object: expr => $($method: ident ( $($arg: expr ),* )).+) => {{
        let inner = $object.write().unwrap();
        let mut inner = inner.borrow_mut();
        inner.$($method($($arg),*)).+
    }};
    ($object: expr => $($method: ident ( $($arg: expr ),* )).+ loop $item: ident {
        $($expr: expr ;)*
    }) => {{
        let inner = $object.write().unwrap();
        let mut inner = inner.borrow_mut();
        for $item in inner.$($method($($arg),*)).+ {
            $($expr ;)*
        }
    }};
}

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