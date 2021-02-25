pub mod entity;

use entity::player::Player;
use rand::Rng;
use std::io;

pub fn game() {
    let mut player: Player = start();
    game_loop(&mut player);
}

fn start() -> Player {
    println!(
        "Welcome to the Kingdom of Textia!\n\
    What's your name, fellow adventurer?"
    );

    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line.");

    Player::new_player(player_name)
}

fn game_loop(player: &mut Player) {
    loop {
        let option: i32 = option_choice_input(
            String::from(
                "\nWhat do you wanna do?\n\
                1. Battle\n\
                2. Heal to full life\n\
                3. See stats\n\
                4. Spend attribute points\n\
                5. Quit\n\
                \n\
                Choose an option: ",
            ),
            1,
            String::from("\nThis isn't a valid option."),
            5,
            String::from("\nThis isn't a valid option."),
        );
        match option {
            1 => player.gain_exp(20 + rand::thread_rng().gen_range(-5, 6), true),
            2 => player.heal_to_full_life(),
            3 => player.see_stats(),
            4 => player.spend_attribute_points(),
            _ => break,
        };
    }
}

fn option_choice_input(
    pre_input_message: String,
    min_input_value: i32,
    less_than_min_error_message: String,
    max_input_value: i32,
    more_than_max_error_message: String,
) -> i32 {
    let option: i32;

    loop {
        println!("{}", pre_input_message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        option = match input.trim().parse() {
            Ok(n) => {
                if n > max_input_value {
                    println!("{}", more_than_max_error_message);
                    continue;
                } else if n < min_input_value {
                    println!("{}", less_than_min_error_message);
                    continue;
                }
                n
            }
            Err(_) => {
                println!("Please input a number!\n");
                continue;
            }
        };

        break;
    }

    option
}
