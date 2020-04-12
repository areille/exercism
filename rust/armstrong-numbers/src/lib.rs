pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let sum: u32 = digits.iter().map(|d| d.pow(digits.len() as u32)).sum();
    sum == num
}
