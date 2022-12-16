fn main() {
    use std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = DirBuilder::new();
    builder.mode(0o777);
}
