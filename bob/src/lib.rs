pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.len() == 0 => "Fine. Be that way!",
        m if is_question(m) && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if is_question(m) => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_yelling(message: &str) -> bool {
    let has_letters = message.chars().filter(|ch| ch.is_alphabetic()).count() > 0;
    has_letters && message.to_uppercase() == message
}

fn is_question(message: &str) -> bool {
    message.trim().ends_with("?")
}
