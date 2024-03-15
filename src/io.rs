pub fn read_line() -> String {
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    choice
}
