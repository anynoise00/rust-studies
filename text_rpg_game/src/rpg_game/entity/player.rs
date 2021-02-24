use super::{stats::Stats, BaseEntity, Entity};
use std::io;

const ATTRIBUTE_POINTS_ON_LVL_UP: i32 = 5;
const HEALTH_POINTS_GROWTH: i32 = 7;

struct Player {
    name: String,
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
            attribute_points: 5,
            total_exp: 0,
            exp_needed_to_level_up: 10,
        }
    }

    pub fn gain_exp(&mut self, exp: i32) {
        self.exp_needed_to_level_up -= exp;
        self.total_exp += exp;

        if self.exp_needed_to_level_up <= 0 {
            self.level_up(-self.exp_needed_to_level_up);
        }
    }

    pub fn spend_attribute_points() {
        // Attribute point spend logic
    }

    fn level_up(&mut self, remnant_exp: i32) {
        println!("You leveled up!");

        self.entity.level += 1;
        self.exp_needed_to_level_up = self.entity.level * 10;
        self.attribute_points += ATTRIBUTE_POINTS_ON_LVL_UP;

        self.entity.stats.health += HEALTH_POINTS_GROWTH;
        self.entity.current_health = self.entity.stats.health;

        if remnant_exp > 0 {
            self.gain_exp(remnant_exp);
        }
    }
}
