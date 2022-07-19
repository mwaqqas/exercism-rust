pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: String = String::new();

    let mut range: Vec<u32> = (end..=start).collect();
    range.reverse();

    for i in range.iter() {
        let current_verse = verse(*i);
        verses.push_str(&current_verse);
        verses.push_str("\n");
    }

    let mut song = verses.trim_end().to_string();
    song.push_str("\n");

    song
}
