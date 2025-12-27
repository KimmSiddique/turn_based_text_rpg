use crate::player::BattlePoints;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Pyrradyn {
    pub stats: TroopStats,
}

impl Pyrradyn {
    const MAX_HEALTH: i32 = 160;
    const DEFENSE: i32 = 12;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Pyrradyn".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Fire,
            ),
        };

        troop.stats.add_move_name("Flame Wheel".to_string(), 2);
        troop.stats.add_move_name("Volcanic Burst".to_string(), 3);
        troop.stats.add_move_name("Ash Storm".to_string(), 4);

        troop
    }
}

impl TroopMoves for Pyrradyn {
    fn move_1(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(2) {
            println!(
                "{0} used: {1}Flame Wheel!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );

            let mut dmg = rand_int(25, 40);
            if *target.get_element() == Element::Rock {
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
                "{0} used: {1}Volcanic Burst!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );

            let mut dmg = rand_int(40, 70);
            if *target.get_element() == Element::Rock {
                dmg *= 2;
            }
            target.take_dmg(dmg);
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(4) {
            println!(
                "{0} used: {1}Ash Storm!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );

            let mut dmg = rand_int(60, 90);
            if *target.get_element() == Element::Rock {
                dmg *= 2;
            }
            target.take_dmg(dmg);

            let heal_amount = rand_int(15, 25);
            self.stats.heal(heal_amount);
            println!(
                "{} recovered {} HP!",
                self.stats.get_troop_name(),
                heal_amount
            );
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
