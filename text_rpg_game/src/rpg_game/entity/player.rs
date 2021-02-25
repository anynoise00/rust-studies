use super::{stats::Stats, BaseEntity, Entity};
use std::io;

const ATTRIBUTE_POINTS_ON_LVL_UP: i32 = 5;
const HEALTH_POINTS_GROWTH: i32 = 7;

pub struct Player {
    pub name: String,
    entity: Entity,
    attribute_points: i32,
    total_exp: i32,
    exp_needed_to_level_up: i32,
}

impl Player {
    pub fn new_player(name: String) -> Player {
        Player {
            name,
            entity: Entity::build_entity_from(
                &BaseEntity::new(
                    String::from("Player"),
                    Stats::new(15, 5, 5, 5),
                    Stats::new(0, 0, 0, 0),
                ),
                1,
            ),
            attribute_points: 10,
            total_exp: 0,
            exp_needed_to_level_up: 10,
        }
    }

    pub fn gain_exp(&mut self, exp: i32, send_exp_message: bool) {
        if send_exp_message {
            println!("You gained {} experience!", exp);
        }
        self.exp_needed_to_level_up -= exp;
        self.total_exp += exp;

        if self.exp_needed_to_level_up <= 0 {
            self.level_up(-self.exp_needed_to_level_up);
        }
    }

    pub fn spend_attribute_points(&mut self) {
        if self.attribute_points <= 0 {
            println!("You dont have any attribute points!");
            return;
        }

        while self.attribute_points > 0 {
            println!(
                "You have {} remaining attribute points. On what do you want to invest it?\n\
                1. Strength\n\
                2. Intelligence\n\
                3. Luck\n\
                4. Cancel\n",
                self.attribute_points
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

            let quantity: i32;

            loop {
                println!("How many points do you wanna spend?");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Your input isn't valid.");
                quantity = match input.trim().parse() {
                    Ok(n) => {
                        if n > self.attribute_points {
                            println!("You don't have that many points!");
                            continue;
                        } else if n <= 0 {
                            println!("You can't do that.");
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

            match option {
                1 => {
                    self.entity.stats.strength =
                        self.spend_points(self.entity.stats.strength, quantity)
                }
                2 => {
                    self.entity.stats.intelligence =
                        self.spend_points(self.entity.stats.intelligence, quantity)
                }
                3 => self.entity.stats.luck = self.spend_points(self.entity.stats.luck, quantity),
                _ => {
                    break;
                }
            }

            if self.attribute_points > 0 {
                println!(
                    "What do you wanna do now?\n\
                1. Spend more, {} remaining\n\
                2. Cancel\n",
                    self.attribute_points
                );
                let option: i32;

                loop {
                    println!("Choose an option: ");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Your input isn't valid.");
                    option = match input.trim().parse() {
                        Ok(n) => {
                            if n < 0 || n > 2 {
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

                    break;
                }
                match option {
                    1 => continue,
                    _ => {
                        println!("Finished spending points.\n");
                        break;
                    }
                }
            } else {
                println!("You have no points reamaining. Finished spending points.\n");
            }
        }
    }

    fn spend_points(&mut self, stat: i32, quantity: i32) -> i32 {
        println!("Attribute point(s) successfully spent!\n");
        self.attribute_points -= quantity;
        stat + quantity
    }

    fn level_up(&mut self, remnant_exp: i32) {
        println!("You leveled up!");

        self.entity.level += 1;
        self.exp_needed_to_level_up = self.entity.level * 10;
        self.attribute_points += ATTRIBUTE_POINTS_ON_LVL_UP;

        self.entity.stats.health += HEALTH_POINTS_GROWTH;
        self.entity.current_health = self.entity.stats.health;

        if remnant_exp > 0 {
            self.gain_exp(remnant_exp, false);
        }
    }
}
