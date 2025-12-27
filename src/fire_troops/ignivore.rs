use crate::player::BattlePoints;
use crate::troop_stats::{Element, TroopMoves, TroopStats};
use crate::utility::{color::*, rand_int::rand_int};

pub struct Ignivore {
    pub stats: TroopStats,
}

impl Ignivore {
    const MAX_HEALTH: i32 = 140;
    const DEFENSE: i32 = 9;

    pub fn new() -> Self {
        let mut troop = Self {
            stats: TroopStats::new(
                "Ignivore".to_string(),
                Self::MAX_HEALTH,
                Self::DEFENSE,
                Element::Fire,
            ),
        };

        troop.stats.add_move_name("Cinder Claw".to_string(), 1);
        troop.stats.add_move_name("Molten Bite".to_string(), 3);
        troop.stats.add_move_name("Hellfire Roar".to_string(), 4);

        troop
    }
}

impl TroopMoves for Ignivore {
    fn move_1(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(1) {
            println!(
                "{0} used: {1}Cinder Claw!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );

            let mut dmg = rand_int(20, 35);
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
                "{0} used: {1}Molten Bite!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );

            let mut dmg = rand_int(35, 55);
            if *target.get_element() == Element::Rock {
                dmg *= 2;
            }
            target.take_dmg(dmg);

            if rand_int(1, 100) <= 25 {
                println!("{} was burned by molten heat!", target.get_troop_name());
                target.take_dmg(rand_int(10, 20));
            }
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }

    fn move_3(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        if player_bp.use_bp(4) {
            println!(
                "{0} used: {1}Hellfire Roar!{2}",
                self.stats.get_troop_name(),
                RED,
                RESET
            );

            let mut dmg = rand_int(50, 80);
            if *target.get_element() == Element::Rock {
                dmg *= 2;
            }
            target.take_dmg(dmg);

            println!("{} took recoil damage!", self.stats.get_troop_name());
            self.stats.take_dmg(rand_int(10, 15));
            return;
        }
        println!("{0}Not enough BP!{1}", BRIGHT_RED, RESET);
    }
}
