use crate::player::BattlePoints;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Blazefang {
    pub stats: TroopStats,
}

impl Blazefang {
    const MAX_HEALTH: i32 = 150;
    const DEFENSE: i32 = 10;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Blazefang".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Fire,
            ),
        };

        troop.stats.add_move_name("Fire-Scorch".to_string(), 2);
        troop.stats.add_move_name("Inferno-Blaze".to_string(), 2);
        troop.stats.add_move_name("Blaze".to_string(), 2);

        troop
    }
}

impl TroopMoves for Blazefang {
    fn move_1(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(2) {
            println!(
                "{0} used: {1}Fire-Scorch!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );
            let mut dmg = 50;
            if *target.get_element() == Element::Rock {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
    fn move_2(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(2) {
            println!(
                "{0} used: {1}Inferno-Blaze!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );
            let mut dmg = rand_int(15, 40);
            if *target.get_element() == Element::Rock {
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
                "{0} used: {1}Blaze!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );
            let heal_amount = rand_int(30, 50);
            self.stats.heal(heal_amount);
            let random_bp_gain = rand_int(0, 2);
            player_bp.gain_bp(random_bp_gain);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
