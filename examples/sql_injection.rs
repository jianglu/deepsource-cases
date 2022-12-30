fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    let args = std::env::args();
    let args: Vec<String> = args.collect();
    let query = format!("SELECT * FROM accounts WHERE custID='{}'", args[1]);
    connection.execute(query).unwrap();
}
