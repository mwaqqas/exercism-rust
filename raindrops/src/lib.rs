pub fn raindrops(n: u32) -> String {
    let sounds = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let mut resultant_sound: String = String::new();

    // check if given number is divisible by any of 3, 5, or 7
    // then append the value of sound to string
    for (num, sound) in sounds {
        if n % num == 0 {
            resultant_sound.push_str(&sound);
        }
    }

    // if the sound is still empty, i.e. not divisible by 3, 5, or 7
    if resultant_sound == String::new() {
        resultant_sound = n.to_string();
    }

    resultant_sound
}
