pub struct Stats {
    pub health: i32,
    pub strength: i32,
    pub intelligence: i32,
    pub luck: i32,
}

impl Stats {
    pub fn summed_with(&self, other: &Stats) -> Stats {
        Stats {
            health: self.health + other.health,
            strength: self.strength + other.strength,
            intelligence: self.intelligence + other.intelligence,
            luck: self.luck + other.luck,
        }
    }

    pub fn multiplied(&self, n: i32) -> Stats {
        Stats {
            health: self.health * n,
            strength: self.strength * n,
            intelligence: self.intelligence * n,
            luck: self.luck * n,
        }
    }

    pub fn new(health: i32, strength: i32, intelligence: i32, luck: i32) -> Stats {
        Stats {
            health,
            strength,
            intelligence,
            luck,
        }
    }
}
