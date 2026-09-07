fn longest(text: &str) -> &str {
    text.split_whitespace().max_by_key(|one| one.len()).unwrap_or("")
}

fn main() {
    println!("{}", longest("a longer sentence here"));
}
