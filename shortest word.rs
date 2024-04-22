fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    let input = "This is a test string with some words";
    
    match shortest_word(input) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the input"),
    }
}
