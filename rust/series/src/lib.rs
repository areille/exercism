pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if len > digits.len() {
        return vec![];
    }
    for i in 0..=digits.len() - len {
        let mut string = String::new();
        for ch in digits.chars().skip(i).take(len) {
            string.push(ch);
        }
        res.push(string);
    }

    res
}
