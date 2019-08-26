fn is_yelling(message: &str) -> bool {
    message
        .chars()
        .filter(|x| x.is_alphabetic())
        .all(|x| x.is_uppercase())
        && message.chars().any(|x| x.is_alphabetic())
}

pub fn reply(message: &str) -> &str {
    let t = message.trim();
    if t.is_empty() {
        "Fine. Be that way!"
    } else {
        match (t.ends_with('?'), is_yelling(t)) {
            (true, true) => "Calm down, I know what I'm doing!",
            (true, _) => "Sure.",
            (_, true) => "Whoa, chill out!",
            _ => "Whatever.",
        }
    }
}
