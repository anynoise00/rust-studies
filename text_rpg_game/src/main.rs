mod rpg_game;

use rpg_game::combat;
use rpg_game::entity::{Entity, stats::Stats};

fn main() {
    let player = Entity::new(String::from("Player"), 1,
        Stats::new(15, 5, 5, 5), Stats::new(5, 1, 1, 1));
}
