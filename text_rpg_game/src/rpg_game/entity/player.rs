use entity::[Entity, BaseEntity];
use std::io;

struct Player {
    name: String,
    entity: Entity,
    attribute_points: i32,
    curr_exp: i32,
    exp_needed_to_level_up: i32,
}

impl Player {
    pub fn new_player(name: String) -> Player {
        Player {
            name,
            entity: Entity::build_entity_from(
                BaseEntity::new(
                    String::from("Player"),
                    Stats::new(15, 5, 5, 5),
                    Stats::new(7, 0, 0, 0),
                ), 1
            ),
            attribute_points: 5,
            curr_exp: 0,
            exp_needed_to_level_up: 10,
        }
    }

    pub fn gain_exp(&mut self, exp: i32) {
        self.exp_needed_to_level_up -= exp;

        if self.exp_needed_to_level_up == 0 {
            
        } else if self.exp_needed_to_level_up < 0 {
            let leftover_exp = exp_needed_to_level_up - exp;
            
        }
    }
}
