// Audit required: Use of VecDeque::make_contiguous RS-A1005
// Security
// In the standard library in Rust before 1.49.0, VecDeque::make_contiguous has a bug that pops the same element more than once under certain condition. This bug could result in a use-after-free or double free.

// This bug has since been fixed, consider upgrading to a newer version Rust to mitigate this issue. To let the DeepSource Rust analyzer know which version of Rust you are using, set the msrv option in your .deepsource.toml file:

// [[analyzers]]
// name = "rust"
// enabled = true

//   [analyzers.meta]
//   msrv = "1.50.0"
// BAD PRACTICE
// use std::collections::VecDeque;

// fn foo(dq: &mut VecDeque<i32>, sz: usize) {
//     for i in 0..sz {
//         dq.push_back(i as _);
//     }
//     dq.make_contiguous();
//     for _ in 0..sz {
//         dq.pop_front();
//     }
// }

// fn main() {
//     let mut dq = VecDeque::with_capacity(2);
//     ab(&mut dq, 2);
//     ab(&mut dq, 3);

//     // this is zero
//     dbg!(dq.len());

//     // this results in a use-after-free
//     // if the VecDeque holds a collection of non-Copy
//     // types, this results in a double-free error as well
//     dbg!(dq.pop_front());
// }
// REFERENCES
// CVE-2020-36318
// CWE-416: Use After Free
use std::collections::VecDeque;

fn foo(dq: &mut VecDeque<i32>, sz: usize) {
    for i in 0..sz {
        dq.push_back(i as _);
    }
    dq.make_contiguous();
    for _ in 0..sz {
        dq.pop_front();
    }
}

fn main() {
    let mut dq = VecDeque::with_capacity(2);
    ab(&mut dq, 2);
    ab(&mut dq, 3);

    // this is zero
    dbg!(dq.len());

    // this results in a use-after-free
    // if the VecDeque holds a collection of non-Copy
    // types, this results in a double-free error as well
    dbg!(dq.pop_front());
}

fn ab(dq: &mut VecDeque<i32>, arg: i32) -> () {
    todo!()
}
