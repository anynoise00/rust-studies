mod rpg_game;

use rpg_game::combat;
use rpg_game::entity::{player::Player, stats::Stats, BaseEntity, Entity};
use std::io;

fn main() {
    let goblin = BaseEntity::new(
        String::from("Goblin"),
        Stats::new(8, 3, 8, 2),
        Stats::new(3, 1, 4, 2),
    );

    let mut player = Player::new_player(String::from("Noise"));

    println!("Hello {}! Welcome to the Kingdom of Textia!", player.name);
    loop {
        println!(
            "What do you wanna do?\n\
        1. Battle\n\
        2. See stats\n\
        3. Spend attribute points\n\
        4. Quit",
        );

        println!("Choose an option: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Your input isn't valid.");
        let option: i32 = match input.trim().parse() {
            Ok(n) => {
                if n < 0 || n > 4 {
                    println!("This isn't a valid option.");
                    continue;
                };
                n
            }
            Err(_) => {
                println!("Please input a number!\n");
                continue;
            }
        };

        match option {
            1 => player.gain_exp(20, true),
            3 => player.spend_attribute_points(),
            _ => break,
        }
    }
}
