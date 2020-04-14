const STACK_EMPTY: &str = "Stack is empty";

fn is_opening_bracket(ch: char) -> bool {
    ch == '{' || ch == '[' || ch == '('
}
fn is_closing_bracket(ch: char) -> bool {
    ch == '}' || ch == ']' || ch == ')'
}

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }
    let mut stack: Vec<char> = Vec::new();
    for ch in string.chars() {
        if is_opening_bracket(ch) {
            stack.push(ch);
        }
        // Exit at this point if stack is empty : opening bracket at start
        if stack.is_empty() && is_closing_bracket(ch) {
            return false;
        }
        if ch == ')' {
            let last = stack.pop().expect(STACK_EMPTY);
            if last == '{' || last == '[' {
                return false;
            }
        } else if ch == '}' {
            let last = stack.pop().expect(STACK_EMPTY);
            if last == '(' || last == '[' {
                return false;
            }
        } else if ch == ']' {
            let last = stack.pop().expect(STACK_EMPTY);
            if last == '{' || last == '(' {
                return false;
            }
        }
    }
    stack.is_empty()
}
