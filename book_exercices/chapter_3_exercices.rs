fn main() {
    let celsius = 0;
    println!("{} degrees celsius is {} degrees fahrenheit.", celsius, celsius_to_fahrenheit(celsius));

    let fahrenheit = 32;
    println!("{} degrees fahrenheit is {} degrees celsius.", fahrenheit, fahrenheit_to_celcius(fahrenheit));
    println!("\n");

    let n = 6;
    println!("The {}° fibonacci  number is {}.", n, nth_fibonacci_number(n));
    println!("\n");

    christmas_carol_lyrics()
}

fn fahrenheit_to_celcius(f_temp: i32) -> i32 {
    (f_temp - 32) * 5/9
}

fn celsius_to_fahrenheit(c_temp: i32) -> i32 {
    (c_temp * 9/5) + 32
}

fn nth_fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut n1 = 0;
        let mut n2 = 1;
    
        let mut i = 2;
        while i < n {
            let x = n1 + n2;
            n1 = n2;
            n2 = x;
    
            i += 1;
        }

        return n1 + n2;
    }
}

fn christmas_carol_lyrics() {
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

    println!("Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree");

    print_repetition_phrase(curr_day);
    curr_day += 1;

    println!("Five gold rings, four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree");

    print_repetition_phrase(curr_day);

    println!("Six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge…");
}

fn print_repetition_phrase(index: usize) {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth"];
    println!("On the {} day of Christmas my true love gave to me", days[index])
}