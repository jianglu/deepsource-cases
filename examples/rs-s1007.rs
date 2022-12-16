// Found potentially incorrect use of bitwise XOR RS-S1007
// Security
// Autofix
// The caret symbol (^) in some languages is used to represent exponentiation. However, in Rust, as in many C-like languages, it represents the bitwise exclusive-or (XOR) operation. It is recommended to carefully vet if the caret is being used for XOR or exponentiation.

// The expression 2^32 in Rust evaluates to the number 34, not 2_i32.pow(32) (or 1 << 32), and such patterns are likely erroneous. It is recommended to use 1 << EXP (for 2^EXP) where EXP is the exponent and the base is 2. For other bases, use the power function from the standard library, such as i32::pow.

// BAD PRACTICE
// 2 ^ 32
// // evaluates to 34, but perhaps the intention was to raise 2 to the power of 32
// RECOMMENDED
// 1_u64 << 32;
// // or
// 2_u64.pow(32);
// REFERENCES
// GCC Bugzilla: GCC should warn about 2^16 and 2^32 and 2^64
fn main() {
    2 ^ 32;
}
