use std::{any::TypeId, error::Error};

struct MyType;

impl Error for MyType {
    fn type_id(&self) -> TypeId {
        // Enable safe casting to `String` by accident.
        TypeId::of::<String>()
    }
}

fn main() {}
