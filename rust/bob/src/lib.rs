pub fn reply(message: &str) -> &str {
    let have_letters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    let is_question: bool = message.trim().chars().last().unwrap() == '?';
    if have_letters && message.to_uppercase() == message {
        if is_question {
            return "Calm down, I know what I'm doing!";
        }
        return "Whoa, chill out!";
    }
    if is_question {
        return "Sure.";
    }
    "Whatever."
}
