use super::super::option_choice_input;
use super::{stats::Stats, BaseEntity, Entity};
use std::io;

const ATTRIBUTE_POINTS_ON_LVL_UP: i32 = 5;
const HEALTH_POINTS_GROWTH: i32 = 7;

pub struct Player {
    entity: Entity,
    attribute_points: i32,
    total_exp: i32,
    exp_needed_to_level_up: i32,
}

impl Player {
    pub fn new_player(name: String) -> Player {
        Player {
            entity: Entity::build_entity_from(
                &BaseEntity::new(name, Stats::new(15, 5, 5, 5), Stats::new(0, 0, 0, 0)),
                1,
            ),
            attribute_points: 10,
            total_exp: 0,
            exp_needed_to_level_up: 10,
        }
    }

    pub fn name(&self) -> &String {
        &self.entity.name
    }

    pub fn gain_exp(&mut self, exp: i32, send_exp_message: bool) {
        if send_exp_message {
            println!("\nYou gained {} experience!", exp);
        }
        self.exp_needed_to_level_up -= exp;
        self.total_exp += exp;

        if self.exp_needed_to_level_up <= 0 {
            self.level_up(-self.exp_needed_to_level_up);
        }
    }

    pub fn see_stats(&self) {
        println!("\nYour stats are:");
        self.entity.info();
        println!(
            "\nTotal experience: {}\n\
            Experience needed to level up: {}
            \n\
            Remaining attribute points: {}",
            self.total_exp, self.exp_needed_to_level_up, self.attribute_points
        );

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
    }

    pub fn spend_attribute_points(&mut self) {
        if self.attribute_points <= 0 {
            println!("\nYou dont have any attribute points!");
            return;
        }

        while self.attribute_points > 0 {
            let option: i32 = option_choice_input(
                format!(
                    "\nYou have {} remaining attribute points. On what do you want to invest it?\n\
                    1. Strength\n\
                    2. Intelligence\n\
                    3. Luck\n\
                    4. Cancel\n\
                    \n\
                    Choose an option: ",
                    self.attribute_points
                ),
                1,
                String::from("\nThis isn't a valid option"),
                4,
                String::from("\nThis isn't a valid option"),
            );

            let quantity: i32 = option_choice_input(
                String::from("\nHow many points do you wanna spend?"),
                1,
                String::from("\nYou can't do that."),
                self.attribute_points,
                String::from("\nYou don't have that many points!"),
            );

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
                let option: i32 = option_choice_input(
                    format!(
                        "\nWhat do you wanna do now?\n\
                        1. Spend more, {} remaining\n\
                        2. Cancel\n\
                        \n\
                        Choose an option: ",
                        self.attribute_points
                    ),
                    1,
                    String::from("\nThis isn't a valid option."),
                    2,
                    String::from("\nThis isn't a valid option."),
                );

                match option {
                    1 => continue,
                    _ => {
                        println!("\nFinished spending points.");
                        break;
                    }
                }
            } else {
                println!("\nYou have no points reamaining. Finished spending points.");
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
