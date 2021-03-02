pub fn pig_latin(word: &String) -> String {
    let vogals = ["a", "e", "i", "o", "u"];
    let mut new_word: String;

    if !vogals.contains(&&word[..1]) {
        new_word = format!("{}-{}{}", &word[1..], &word[..1], "ay".to_string());
    } else {
        new_word = word.clone();
        new_word.push_str("-hay");
    }

    new_word
}
