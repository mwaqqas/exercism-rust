pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    match list.len() {
        0 => {}
        1 => {
            let line = format!("And all for the want of a {}.", list[0]);
            proverb.push_str(&line);
        }
        _ => {
            for (idx, word) in list.iter().enumerate() {
                // check if it is the last line
                match idx == list.len() - 1 {
                    false => {
                        let line =
                            format!("For want of a {} the {} was lost.\n", word, list[idx + 1]);
                        proverb.push_str(&line)
                    }
                    true => {
                        let line = format!("And all for the want of a {}.", list[0]);
                        proverb.push_str(&line)
                    }
                }
            }
        }
    }

    proverb
}
