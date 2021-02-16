pub mod stats;

pub struct Entity {
    name: String,
    current_health: u16,
    level: u16,
    pub stats: stats::Stats,
    stat_growth: stats::Stats,
}

impl Entity {
    pub fn new(name: String, level: u16, base_stats: stats::Stats, stat_growth: stats::Stats) -> Entity {
        let mut e = Entity {
            name,
            current_health: 1,
            level: 1,
            stats: base_stats,
            stat_growth,
        };

        e.level_up(level - e.level);

        e
    }
    
    pub fn info(&self) {
        println!("{}, <Lv. {}>\nHP: {}/{}", self.name,
        self.level, self.current_health, self.stats.health);
        println!("STR: {}, INT: {}, LCK: {}", self.stats.strength,
        self.stats.intelligence, self.stats.luck)
    }

    pub fn damaged(&self, damage: u16) {
        self.current_health -= damage;
    }

    fn level_up(&mut self, levels: u16) {
        self.stats = self.stats.sum_with(&self.stat_growth.multiplied(levels));
        self.current_health = self.stats.health;
        self.level += levels;
    }
}