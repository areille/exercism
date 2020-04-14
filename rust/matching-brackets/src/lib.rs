fn opposite(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => unreachable!(),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in string.chars() {
        match ch {
            '{' | '[' | '(' => stack.push(ch),
            ')' | '}' | ']' => {
                if stack.pop() != Some(opposite(ch)) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
