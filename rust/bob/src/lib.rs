fn is_yelling(message: &str) -> bool {
    message
        .chars()
        .filter(|x| x.is_alphabetic())
        .all(|x| x.is_uppercase())
        && message.chars().any(|x| x.is_alphabetic())
}

pub fn reply(message: &str) -> &str {
    let t = message.trim();
    match (t.len(), t.ends_with('?'), is_yelling(t)) {
        (0, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
