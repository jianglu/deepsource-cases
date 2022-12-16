fn main() {
    unsafe {
        let p = libc::malloc(100) as *mut u32;
        libc::free(p as _);
        *p = 99;
    }
}
