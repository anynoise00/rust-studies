pub mod stats;

use stats::Stats;

pub struct Entity {
    name: String,
    current_health: i32,
    level: i32,
    pub stats: Stats,
}

impl Entity {
    pub fn build_entity_from(base_entity: &BaseEntity, level: i32) -> Entity {
        let leveled_stats: Stats = base_entity
            .base_stats
            .summed_with(&base_entity.stat_growth.multiplied(level - 1));

        Entity {
            name: base_entity.name.clone(),
            level,
            current_health: leveled_stats.health,
            stats: leveled_stats,
        }
    }
    pub fn info(&self) {
        println!(
            "{}, <Lv. {}>\nHP: {}/{}",
            self.name, self.level, self.current_health, self.stats.health
        );
        println!(
            "STR: {}, INT: {}, LCK: {}",
            self.stats.strength, self.stats.intelligence, self.stats.luck
        )
    }

    pub fn take_damage(&mut self, value: i32) {
        self.current_health -= value;
    }
}

pub struct BaseEntity {
    name: String,
    base_stats: Stats,
    stat_growth: Stats,
}

impl BaseEntity {
    pub fn new(name: String, base_stats: Stats, stat_growth: Stats) -> BaseEntity {
        BaseEntity {
            name,
            base_stats,
            stat_growth,
        }
    }
}
