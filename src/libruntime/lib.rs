//! Runtime

/// External crates
extern crate libc;

/// Standard libraries
use std::thread::{spawn, JoinHandle};

pub struct Process {
    pub thread: JoinHandle<()>,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libruntime__process(f: extern "C" fn() -> libc::c_void) -> *mut Process {
    let process = Process {
        thread: spawn(move || {
            // Call the function
            f();
        }),
    };
    Box::into_raw(Box::new(process))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libruntime__process_join(process: *mut Process) {
    let process = unsafe { Box::from_raw(process) };
    process.thread.join().expect("runtime error: synchronization failed");
}
