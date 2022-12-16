fn main() {
    use http::header::{HeaderMap, SERVER};

    let mut map = HeaderMap::new();
    map.insert(SERVER, "Apache/2.4.1 (Unix)".parse().unwrap());
}
