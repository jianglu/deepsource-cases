fn main() {
    let _ = (0..5).fold(false, |acc, x| acc || x > 2);

    // is better written as
    // let _ = (0..5).any(|x| x > 2);

    std::env::var("RUST_BACKTRACE").unwrap();
    // elsewhere ...
    std::env::remove_var("RUST_BACKTRCAE"); // oops, misspelled

    unsafe {
        let p = libc::malloc(100) as *mut u32;
        libc::free(p as _);
        *p = 99;
    }

    if foo() == Some(1) {}
}

fn foo() -> Option<i32> {
    Some(2)
}
