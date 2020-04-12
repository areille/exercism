pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let is_divider = |a, b| b != 0 && a % b == 0;
    (0..limit)
        .filter(|&x| factors.iter().any(|&f| is_divider(x, f)))
        .sum()
}
