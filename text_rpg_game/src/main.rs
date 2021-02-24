mod rpg_game;

use rpg_game::combat;
use rpg_game::entity::{stats::Stats, BaseEntity, Entity};

fn main() {
    let goblin = BaseEntity::new(
        String::from("Goblin"),
        Stats::new(8, 3, 8, 2),
        Stats::new(3, 1, 4, 2),
    );
}
