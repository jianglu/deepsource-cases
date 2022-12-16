// Detected conversion of raw slice to ptr RS-S1012
// Security
// Conversion from a raw slice to ptr results in undefined behavior.

// Consider using the following conversion methods:

// std::ptr::slice_from_raw_parts to convert a ptr and len pair into a raw slice
// std::slice::as_ptr to convert a raw slice to a ptr
// BAD PRACTICE
// let x = &[10, 10, 20];
// let foo = x as *const [i32] as *const i32;
// REFERENCE
// Notes on Safety & Undefined Behaviour in Rust
fn main() {
    let x = &[10, 10, 20];
    let foo = x as *const [i32] as *const i32;
}
