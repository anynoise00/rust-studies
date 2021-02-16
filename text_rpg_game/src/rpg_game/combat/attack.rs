const physical_attack: Attack = Attack::new(String::from("Basic Slash"),
    DamageBonus::new(2, 0.2), AttackType::Physical);
const magical_attack: Attack = Attack::new(String::from("Basic Fireball"),
    DamageBonus::new(3, 0.1), AttackType::Magical);

pub enum AttackType {
    Physical,
    Magical,
}

pub struct Attack {
    name: String,
    damage_bonus: DamageBonus,
    attack_type: AttackType,
}

impl Attack {
    pub fn new(name: String, damage_bonus: DamageBonus, attack_type: AttackType) -> Attack {
        Attack {
            name,
            damage_bonus,
            attack_type,
        }
    }

    pub fn info(&self) {
        println!("Name: {}\nDamage Bonus: {}% chance to +{}\nAttack Type: {}",
            self.name,
            self.damage_bonus.chance * 100.0,
            self.damage_bonus.bonus,
            match self.attack_type {
                AttackType::Physical => "Physical",
                AttackType::Magical => "Magical",
            }
        )
    }
}

struct DamageBonus {
    bonus: u16,
    chance: f32,
}

impl DamageBonus {
    pub fn new(bonus: u16, chance: f32) -> DamageBonus {
        DamageBonus {
            bonus,
            chance,
        }
    }
}