use crate::player::Player;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Aquashock {
    pub stats: TroopStats,
}

impl Aquashock {
    const MAX_HEALTH: i32 = 140;
    const DEFENSE: i32 = 12;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Aquashock".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Water,
            ),
        };

        troop.stats.add_move_name("Hydro Burst".to_string(), 2);
        troop.stats.add_move_name("Aqua Pulse".to_string(), 3);
        troop.stats.add_move_name("Tidal Wave".to_string(), 2);

        troop
    }
}

impl TroopMoves for Aquashock {
    fn move_1(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(2) {
            println!(
                "{0} used: {1}Hydro Burst!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let mut dmg = rand_int(25, 40);
            if *target.get_element() == Element::Fire {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_2(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(3) {
            println!(
                "{0} used: {1}Aqua Pulse!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let mut dmg = rand_int(35, 55);
            if *target.get_element() == Element::Fire {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(2) {
            println!(
                "{0} used: {1}Tidal Wave!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let dmg = rand_int(20, 35);
            let heal_amount = rand_int(10, 20);
            target.take_dmg(dmg);
            self.stats.heal(heal_amount);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
