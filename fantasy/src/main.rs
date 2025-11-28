/*
    Unfinished code practice for Rust programming
    ISSUE: u32 overflow
    WARN: Unused variable health for monster
    DATE: 28.11.2025
*/

#[derive(Debug)]
enum Class {
    Warrior,
    Wizard,
    Ranger,
    Fairy
}
#[derive(Debug)]
struct Player {
    health: u32,
    class: Class
}

struct Monster {
    flying: bool,
    health: u32,
    attack_modifier: u32
}

trait HasHealth {
    fn change_health(&mut self) -> &mut u32;
}

trait DamageAbsorption {
    fn absorb_damage(&self) -> u32;
}

impl HasHealth for Player {
    fn change_health(&mut self) -> &mut u32 {
        &mut self.health
    }
}

impl DamageAbsorption for Player {
    fn absorb_damage(&self) -> u32 {
        match self.class {
            Class::Warrior => 30,
            Class::Wizard => 0,
            Class::Ranger => 5,
            Class::Fairy => 90
        }
    }
}

trait IsFlying {
    fn flight_check(&self) -> bool;
}

trait MonsterBehaviour<T: HasHealth + DamageAbsorption + std::fmt::Debug>: IsFlying {
    fn attack_modifier(&self) -> u32;

    fn attack_player(&self, target: &mut T) {
        let base = if self.flight_check() { 20 } else { 10 };
        let absorption_shield = target.absorb_damage();
        if absorption_shield > base * self.attack_modifier() {
            println!("Attack failed on Player {:?}!", target);
        } else {
            *target.change_health() -= base * self.attack_modifier();
        }
    }
}

impl IsFlying for Monster {
    fn flight_check(&self) -> bool {
        self.flying
    }
}

impl<T: HasHealth + DamageAbsorption + std::fmt::Debug> MonsterBehaviour<T> for Monster {
    fn attack_modifier(&self) -> u32 {
        self.attack_modifier
    }
}

fn main() {
    // Single monster attacking a single player
    let mut player = Player { health: 100, class: Class::Wizard };
    let monster = Monster { flying: true, health: 100, attack_modifier: 1 };
    println!("Player HP: {}", player.health);
    monster.attack_player(&mut player);
    println!("Player HP: {}", player.health);
    // Multiple monsters attacking multiple players at once
    // Wizard, Ranger, Fairy
    let mut players: Vec<Player> = vec![Player { health: 100, class: Class::Wizard }, Player { health: 100, class: Class::Ranger }, Player { health: 100, class: Class::Fairy }];
    let monsters: Vec<Monster> = vec![Monster {flying: true, health: 100, attack_modifier: 4}, Monster{flying: true, health: 100, attack_modifier: 1}, Monster{flying: false, health: 100, attack_modifier: 1}];
    players.iter().for_each(|p| println!("Player HP: {}, Player Class: {:?}", p.health, p.class));
    // iter_mut -> returns &mut T
    // iter -> &T
    players.iter_mut().for_each(|p| {
        monsters.iter().for_each(|m| {
            m.attack_player(p)
        });
    });

    players.iter().for_each(|p| println!("Player HP: {}, Player Class: {:?}", p.health, p.class));
}
