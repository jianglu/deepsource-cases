// Detected conversion of *const to *mut RS-S1011
// Security
// Converting *const to *mut works in safe code. However, mutating such a pointer can result in undefined behavior. Such situations can only occur in unsafe code, because dereferencing pointers is an unsafe operation.

// BAD PRACTICE
// let x = 10;
// let foo = &x as *const i32 as *mut i32;
// REFERENCE
// Notes on Safety & Undefined Behaviour in Rust
fn main() {
    let x = 10;
    let foo = &x as *const i32 as *mut i32;
}
