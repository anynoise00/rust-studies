#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter_alaska = Coin::Quarter(UsState::Alaska);
    
    println!("A penny value in cents is {}", value_in_cents(penny));
    println!("A nickel value in cents is {}", value_in_cents(nickel));
    println!("A dime value in cents is {}", value_in_cents(dime));
    println!("A quarter value in cents is {}\n", value_in_cents(quarter_alaska));

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("The count is {}\n", count);

    let five = Some(5);
    let none = plus_one(&None);
    println!("{:?} plus one is {:?}", five, plus_one(&five));
    println!("None is {:?}", none);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(5) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}