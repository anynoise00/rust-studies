pub fn christmas_carol_lyrics() {
    let mut curr_day = 0;

    print_repetition_phrase(curr_day);
    curr_day += 1;

    println!("A partridge in a pear tree");

    print_repetition_phrase(curr_day);
    curr_day += 1;

    println!("Two turtle doves and a partridge in a pear tree");
    print_repetition_phrase(curr_day);
    curr_day += 1;

    println!("Three French hens, two turtle doves and a partridge in a pear tree");

    print_repetition_phrase(curr_day);
    curr_day += 1;

    println!(
        "Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree"
    );

    print_repetition_phrase(curr_day);
    curr_day += 1;

    println!(
        "Five gold rings, four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree"
    );

    print_repetition_phrase(curr_day);

    println!(
        "Six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge…"
    );
}

fn print_repetition_phrase(index: usize) {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth"];
    println!(
        "On the {} day of Christmas my true love gave to me",
        days[index]
    )
}
