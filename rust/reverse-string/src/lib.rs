pub fn reverse(input: &str) -> String {
    input
        .chars()
        .map(|c| c.to_string())
        .rev()
        .collect::<Vec<_>>()
        .join("")
    // Better solution
    // input.chars().rev().collect()
}
