use rand::Rng;

fn main() {
    conditional();
    println!("\n");

    basic_loop();
    println!("\n");

    while_loop();
    println!("\n");

    for_loop();
    println!("\n");
}

fn conditional() {
    let number = rand::thread_rng().gen_range(0, 10);
    let number_is = if number % 2 == 0 {"even"} else {"odd"};

    if number > 5 {
        println!("{} is higher than 5, condition was true.", number);
    } else {
        println!("{} is lower than 5, condition was false.", number);
    }

    println!("{} is an {} number.", number, number_is);
}

fn basic_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LAUNCH!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}