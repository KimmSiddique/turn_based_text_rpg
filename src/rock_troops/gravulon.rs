use crate::player::Player;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Gravulon {
    pub stats: TroopStats,
}

impl Gravulon {
    const MAX_HEALTH: i32 = 180;
    const DEFENSE: i32 = 18;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Gravulon".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Rock,
            ),
        };

        troop.stats.add_move_name("Rockfall".to_string(), 2);
        troop.stats.add_move_name("Earthen Heal".to_string(), 3);
        troop.stats.add_move_name("Meteor Strike".to_string(), 4);

        troop
    }
}

impl TroopMoves for Gravulon {
    fn move_1(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(2) {
            println!(
                "{0} used: {1}Rockfall!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let mut dmg = rand_int(20, 35);
            if *target.get_element() == Element::Water {
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
                "{0} used: {1}Earthen Heal!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let heal_amount = rand_int(30, 45);
            self.stats.heal(heal_amount);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player: &mut Player, target: &mut TroopStats) {
        if player.use_bp(4) {
            println!(
                "{0} used: {1}Meteor Strike!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let mut dmg = rand_int(45, 60);
            if *target.get_element() == Element::Water {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
