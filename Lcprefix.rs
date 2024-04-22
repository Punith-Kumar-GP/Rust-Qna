fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let mut prefix = strs[0].clone();
    
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    
    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);
}
