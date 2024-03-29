pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '{' => brackets.push('}'),
            '[' => brackets.push(']'),
            '(' => brackets.push(')'),
            '}' | ']' | ')' => {
                if Some(c) != brackets.pop() {
                    return false;
                }
            }
            _ => (),
        };
    }

    brackets.is_empty()
}
