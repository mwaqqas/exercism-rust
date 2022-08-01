pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with('?') && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with('?') => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_yelling(message: &str) -> bool {
    let has_letters = message.chars().filter(|ch| ch.is_alphabetic()).count() > 0;
    has_letters && message.to_uppercase() == message
}
