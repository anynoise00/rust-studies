mod attack;

use super::entity;

fn deal_damage(attacker: &entity::Entity, target: &mut entity::Entity, attack_type: attack::AttackType) {
    match attack_type {
        attack::AttackType::Physical => {
            target.damaged()
        },
        attack::AttackType::Magical => {

        },
    }
}