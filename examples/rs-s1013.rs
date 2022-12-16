// Detected conversion between differently sized raw slices RS-S1013
// Security
// Conversion between raw slices of differently sized types is undefined behaviour, because the length of the pointer is not converted using as.
// BAD PRACTICE
// let x = &[10, 20];
// let foo = x as *const [i32] as *const [u8];
// REFERENCE
// Notes on Safety & Undefined Behaviour in Rust
fn main() {
    let x = &[10, 20];
    let foo = x as *const [i32] as *const [u8];
}
