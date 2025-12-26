use crate::player::Player;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Torrendor {
    pub stats: TroopStats,
}

impl Torrendor {
    const MAX_HEALTH: i32 = 130;
    const DEFENSE: i32 = 8;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Torrendor".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Water,
            ),
        };

        troop.stats.add_move_name("Raging Current".to_string(), 2);
        troop.stats.add_move_name("Maelstrom".to_string(), 3);
        troop.stats.add_move_name("Steam Burst".to_string(), 4);

        troop
    }
}

impl TroopMoves for Torrendor {
    fn move_1(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(2) {
            println!(
                "{0} used: {1}Raging Current!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let mut dmg = rand_int(30, 45);
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
                "{0} used: {1}Maelstrom!{2}",
                self.stats.get_troop_name(),
                BLUE,
                RESET
            );

            let mut dmg = rand_int(40, 55);
            if *target.get_element() == Element::Fire {
                dmg *= 2;
            }
            target.take_dmg(dmg);

            self.stats.take_dmg(10);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(4) {
            println!(
                "{0} used: {1}Steam Burst!{2}",
                self.stats.get_troop_name(),
                BLUE,
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
