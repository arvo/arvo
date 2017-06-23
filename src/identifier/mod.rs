//! Identification

use std::cmp::{Eq, PartialEq};
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use super::libc::c_char;

/// The `GLOBAL_ID` is used to generate unique integers. It is thread safe.
static GLOBAL_ID: AtomicUsize = ATOMIC_USIZE_INIT;

/// The `Id` trait is implemented by nodes that can provide a numerical id.
/// This id must be globally unique.
pub trait Id {
    fn id(&self) -> usize;
}

/// The `Name` trait is implemented by nodes that can provide a human-readable
/// name. This name does not have to be globally unique.
pub trait Name {
    fn name(&self) -> String;
}

/// The `Identify` trait is implemented by nodes that can provide an
/// `Identifier`.
pub trait Identify {
    fn identify(&self) -> Identifier;
}

/// The `Symbolise` trait is implemented by nodes that can provide a `Symbol`.
pub trait Symbolise {
    fn symbolise(&self) -> Symbol;
}

/// An alias for a vector of `Identifier` structs.
pub type Identifiers = Vec<Identifier>;

/// All components that require identification must use the `Identifier`
/// struct. The globally unique id is automatically generated and the empty
/// string is a valid name.
#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Identifier {
    pub id: usize,
    pub name: String,
}

impl Identifier {
    /// Build a new `Identifier`. This will automatically generate a globally
    /// unique id.
    ///
    /// # Return
    /// A new `Identifier`.
    pub fn id() -> Identifier {
        Identifier {
            id: GLOBAL_ID.fetch_add(1, Ordering::SeqCst),
            name: String::new(),
        }
    }

    /// Build a new `Identifier` with a name. This will automatically generate
    /// a globally unique id.
    ///
    /// # Arguments
    /// * `name` - The name of this `Identifier`.
    ///
    /// # Return
    /// A new `Identifier`.
    pub fn name<N>(name: N) -> Identifier
        where N: Into<String>
    {
        Identifier {
            id: GLOBAL_ID.fetch_add(1, Ordering::SeqCst),
            name: name.into(),
        }
    }
}

/// An `Identifier` exposes a globally unique id.
impl Id for Identifier {
    fn id(&self) -> usize {
        self.id
    }
}

impl<T> Id for T
    where T: Identify
{
    fn id(&self) -> usize {
        self.identify().id()
    }
}

/// An `Identifier` exposes a human-readable name.
impl Name for Identifier {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<T> Name for T
    where T: Identify
{
    fn name(&self) -> String {
        self.identify().name()
    }
}

/// An `Identifier` can be created from an FFI char pointer.
impl From<*mut c_char> for Identifier {
    fn from(ident: *mut c_char) -> Identifier {
        unsafe { Identifier::name(CStr::from_ptr(ident).to_string_lossy().into_owned()) }
    }
}

/// An `Identifier` can be tested for equality. Equality is determined by the
/// numerical id. The human-readble name is ignored.
impl Eq for Identifier {}

/// An `Identifier` can be tested for partial equality. Partial equality is
/// determined by the numerical id. The human-readble name is ignored.
impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// An `Identifier` is hashed using its numerical id.
impl Hash for Identifier {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.id.hash(state)
    }
}

/// All components that require identification by naming an element in the
/// source code must use the `Symbol` struct. The `Symbol` should not be used
/// to identify a component, but rather to name it with a name that is
/// directly reflected in the source code.
#[derive(Clone)]
pub struct Symbol {
    identifier: Identifier,
}

impl Symbol {
    pub fn new<N>(name: N) -> Symbol
        where N: Into<String>
    {
        Symbol::from(Identifier::name(name))
    }
}

impl Identify for Symbol {
    fn identify(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl From<Identifier> for Symbol {
    fn from(identifier: Identifier) -> Symbol {
        Symbol {
            identifier: identifier
        }
    }
}

/// Static identifiers that are intrinsic to the language.
lazy_static! {

    /// Functions.
    pub static ref FN_LIBRUNTIME_PROCESS: Symbol = Symbol::new("__libruntime__process");
    pub static ref FN_LIBRUNTIME_PROCESS_FORMAL1: Symbol = Symbol::new("f");
    pub static ref FN_LIBRUNTIME_PROCESS_JOIN: Identifier = Identifier::name("__libruntime__process_join");
    pub static ref FN_LIBRUNTIME_PROCESS_JOIN_FORMAL1: Symbol = Symbol::new("process");

}
