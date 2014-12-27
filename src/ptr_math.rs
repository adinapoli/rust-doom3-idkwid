
pub fn ptr_add<T>(ptr: *const T, offset: u64) -> *const T {
    let p = ptr as u64;
    (p + offset) as *const T
}

pub fn ptr_add_mut<T>(ptr: *mut T, offset: u64) -> *mut T {
    let p = ptr as u64;
    (p + offset) as *mut T
}
