use crate::player::Player;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Glacivern {
    pub stats: TroopStats,
}

impl Glacivern {
    const MAX_HEALTH: i32 = 170;
    const DEFENSE: i32 = 16;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Glacivern".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Water,
            ),
        };

        troop.stats.add_move_name("Frostbite".to_string(), 2);
        troop.stats.add_move_name("Glacial Heal".to_string(), 3);
        troop.stats.add_move_name("Cryo Surge".to_string(), 4);

        troop
    }
}

impl TroopMoves for Glacivern {
    fn move_1(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(2) {
            println!(
                "{0} used: {1}Frostbite!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let mut dmg = rand_int(20, 35);
            if *target.get_element() == Element::Fire {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_2(&mut self, player: &mut Player, _target: &mut TroopStats) {
        if player.use_bp(3) {
            println!(
                "{0} used: {1}Glacial Heal!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let heal_amount = rand_int(25, 45);
            self.stats.heal(heal_amount);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(4) {
            println!(
                "{0} used: {1}Cryo Surge!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let mut dmg = rand_int(40, 55);
            if *target.get_element() == Element::Fire {
                dmg *= 2;
            }
            target.take_dmg(dmg);

            let heal_amount = rand_int(10, 20);
            self.stats.heal(heal_amount);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
