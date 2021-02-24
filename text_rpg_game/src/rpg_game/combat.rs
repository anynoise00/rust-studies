use super::entity;
use rand::Rng;

enum AttackType {
    Physical,
    Magical,
}

fn battle(attacker: &entity::Entity, target: &mut entity::Entity, attack_type: AttackType) {
    let mut dmg: i32 = 0;

    match attack_type {
        AttackType::Physical => {
            dmg += calculate_damage(attacker.stats.strength, target.stats.strength, is_crit());
            dmg += calculate_bonus_damage(attacker.stats.strength, attacker.stats.luck);
        }
        AttackType::Magical => {
            dmg += calculate_damage(
                attacker.stats.intelligence,
                target.stats.intelligence,
                is_crit(),
            );
            dmg += calculate_bonus_damage(attacker.stats.intelligence, attacker.stats.luck);
        }
    }

    target.take_damage(dmg);
}

fn calculate_damage(attacker_dmg_stat: i32, target_dmg_stat: i32, crit: bool) -> i32 {
    let mut actual_damage: i32;

    actual_damage = attacker_dmg_stat as i32 - target_dmg_stat as i32;
    actual_damage = if actual_damage <= 0 { 1 } else { actual_damage };
    actual_damage = if crit {
        actual_damage * 2
    } else {
        actual_damage
    };

    actual_damage.max(0) as i32
}

fn calculate_bonus_damage(attacker_dmg_stat: i32, attacker_luck_stat: i32) -> i32 {
    if attacker_luck_stat > 0
        && attacker_luck_stat < 100
        && rand::thread_rng().gen_range(1, 100) < attacker_luck_stat
    {
        return ((attacker_dmg_stat as f32).ceil() / 10.0) as i32;
    }

    return 0;
}

fn is_crit() -> bool {
    rand::thread_rng().gen_range(1, 11) <= 1
}
