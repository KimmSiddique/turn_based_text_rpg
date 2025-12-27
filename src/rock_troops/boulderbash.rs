use crate::player::BattlePoints;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Boulderbash {
    pub stats: TroopStats,
}

impl Boulderbash {
    const MAX_HEALTH: i32 = 160;
    const DEFENSE: i32 = 14;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Boulderbash".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Rock,
            ),
        };

        troop.stats.add_move_name("Stone Slam".to_string(), 2);
        troop.stats.add_move_name("Pebble Barrage".to_string(), 3);
        troop.stats.add_move_name("Rock Shield".to_string(), 2);

        troop
    }
}

impl TroopMoves for Boulderbash {
    fn move_1(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(2) {
            println!(
                "{0} used: {1}Stone Slam!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let mut dmg = rand_int(25, 40);
            if *target.get_element() == Element::Water {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_2(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(3) {
            println!(
                "{0} used: {1}Pebble Barrage!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let mut dmg = rand_int(35, 50);
            if *target.get_element() == Element::Water {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player_bp: &mut BattlePoints, _target: &mut TroopStats) {
        if player_bp.use_bp(2) {
            println!(
                "{0} used: {1}Rock Shield!{2}",
                self.stats.get_troop_name(),
                YELLOW,
                RESET
            );

            let heal_amount = rand_int(20, 35);
            self.stats.heal(heal_amount);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
