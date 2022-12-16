extern crate rayon;

use rayon::join;
use std::sync::Arc;

fn main() {
    let a1 = Arc::new(0);
    let mut a2 = a1.clone();
    join(
        // thread 1
        || {
            let _: u32 = *a1;
            drop(a1);
        },
        // thread 2
        || loop {
            match Arc::get_mut(&mut a2) {
                None => {}
                Some(m) => {
                    *m = 1u32;
                    return;
                }
            }
        },
    );
}
