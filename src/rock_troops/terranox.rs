use crate::player::Player;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Terranox {
    pub stats: TroopStats,
}

impl Terranox {
    const MAX_HEALTH: i32 = 135;
    const DEFENSE: i32 = 10;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Terranox".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Rock,
            ),
        };

        troop.stats.add_move_name("Seismic Crush".to_string(), 2);
        troop.stats.add_move_name("Tremor Fist".to_string(), 3);
        troop.stats.add_move_name("Dust Storm".to_string(), 4);

        troop
    }
}

impl TroopMoves for Terranox {
    fn move_1(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(2) {
            println!(
                "{0} used: {1}Seismic Crush!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let mut dmg = rand_int(30, 45);
            if *target.get_element() == Element::Water {
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
                "{0} used: {1}Tremor Fist!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let mut dmg = rand_int(40, 55);
            if *target.get_element() == Element::Water {
                dmg *= 2;
            }
            target.take_dmg(dmg);

            self.stats.take_dmg(5);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(4) {
            println!(
                "{0} used: {1}Dust Storm!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let dmg = rand_int(20, 35);
            target.take_dmg(dmg);
            player.gain_bp(2);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
