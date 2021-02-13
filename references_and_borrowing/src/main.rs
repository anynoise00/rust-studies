fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("goodbye");

    let len = calculate_length(&s1);
    change(&mut s2);

    println!("The length of '{}' is {}.", s1, len);

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s2; // no problem
    println!("{}", r3);

    let hello = String::from("hello beautiful world");
    let word = first_word(&hello[..]);
    println!("The first word from the phrase '{}' is '{}'.", hello, word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}