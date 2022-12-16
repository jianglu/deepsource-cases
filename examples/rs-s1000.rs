// Potentially unsafe usage of Arc::get_mut RS-S1000
// Security
// In the standard library in Rust before 1.29.0, there is weak synchronization in the Arc::get_mut method. This synchronization issue can be lead to memory safety issues through race conditions.

// This bug has since been fixed, consider upgrading to a newer version of Rust to mitigate this issue.

// BAD PRACTICE
// extern crate rayon;

// use std::sync::Arc;
// use rayon::join;

// fn main() {
//     let a1 = Arc::new(0);
//     let mut a2 = a1.clone();
//     join(
//         // thread 1
//         || {
//             let _: u32 = *a1;
//             drop(a1);
//         },

//         // thread 2
//         || loop {
//             match Arc::get_mut(&mut a2) {
//                 None => {}
//                 Some(m) => {
//                     *m = 1u32;
//                     return;
//                 }
//             }
//         },
//     );
// }
// The above code sample results in a data race, between a read in the first thread and a write in the second thread, because Arc::get_mut only uses a relaxed read of the strong counter.

// REFERENCES
// CVE-2018-25008
