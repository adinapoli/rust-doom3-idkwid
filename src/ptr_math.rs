
extern crate libc;
use libc::{size_t};

pub fn ptr_add<T>(ptr: *const T, offset: size_t) -> *const T {
    let p = ptr as size_t;
    (p + offset) as *const T
}

pub fn ptr_add_mut<T>(ptr: *mut T, offset: size_t) -> *mut T {
    let p = ptr as size_t;
    (p + offset) as *mut T
}
