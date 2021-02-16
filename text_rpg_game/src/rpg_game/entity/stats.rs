pub struct Stats {
    pub health: u16,
    pub strength: u16,
    pub intelligence: u16,
    pub luck: u16,
}

impl Stats {
    pub fn sum_with(&self, other: &Stats) -> Stats {
        Stats {
            health: self.health + other.health,
            strength: self.strength + other.strength,
            intelligence: self.intelligence + other.intelligence,
            luck: self.luck + other.luck,
        }
    }

    pub fn multiplied(&self, n: u16) -> Stats {
        Stats {
            health: self.health * n,
            strength: self.strength * n,      
            intelligence: self.intelligence * n,
            luck: self.luck * n,
        }
    }

    pub fn new(health: u16, strength: u16, intelligence: u16, luck: u16) -> Stats {
        Stats {
            health,
            strength,
            intelligence,
            luck,
        }
    }
}