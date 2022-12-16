#[derive(Debug)]
enum MyEnum {
    DroppedTwice(Box<i32>),
    PanicOnDrop,
}

impl Drop for MyEnum {
    fn drop(&mut self) {
        match self {
            MyEnum::DroppedTwice(_) => println!("Dropping!"),
            MyEnum::PanicOnDrop => panic!(),
        }
    }
}

fn main() {
    let v = vec![MyEnum::DroppedTwice(Box::new(123)), MyEnum::PanicOnDrop];

    // results in a double-free error
    Vec::from_iter(v.into_iter().take(0));
}
