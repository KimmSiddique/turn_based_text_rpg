use crate::player::BattlePoints;
use crate::utility::color::*;

pub trait TroopMoves {
    fn move_1(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats);
    fn move_2(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats);
    fn move_3(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats);
}

pub struct TroopStats {
    troop_name: String,
    max_health: i32,
    health: i32,
    defense: i32,
    element: Element,
    is_alive: bool,
    move_names: Vec<(String, i32)>,
}

#[derive(PartialEq, Eq)]
pub enum Element {
    Rock,
    Water,
    Fire,
}

impl TroopStats {
    pub fn new(troop_name: String, max_health: i32, defense: i32, element: Element) -> Self {
        Self {
            troop_name,
            max_health,
            health: max_health,
            defense,
            element,
            is_alive: true,
            move_names: Vec::new(),
        }
    }
    // Getters & Setters:
    pub fn get_troop_name(&self) -> &str {
        &self.troop_name
    }
    pub fn get_max_health(&self) -> &i32 {
        &self.max_health
    }
    pub fn get_health(&self) -> &i32 {
        &self.health
    }
    pub fn get_defense(&self) -> &i32 {
        &self.defense
    }
    pub fn get_element(&self) -> &Element {
        &self.element
    }
    pub fn get_is_alive(&self) -> bool {
        self.is_alive
    }
    pub fn get_element_to_string(&self) -> String {
        match &self.element {
            Element::Fire => "Fire".to_string(),
            Element::Water => "Water".to_string(),
            Element::Rock => "Rock".to_string(),
        }
    }
    pub fn add_move_name(&mut self, move_name: String, bp_cost: i32) {
        self.move_names.push((move_name, bp_cost));
    }
    pub fn display_moves(&self) {
        for (move_name, bp) in self.move_names.iter() {
            println!("{} | BP required: {}", move_name, bp);
        }
    }

    pub fn take_dmg(&mut self, dmg: i32) {
        let scaling: f32 = 100.0 / (100.0 + self.defense as f32);
        let actual_dmg: i32 = (dmg as f32 * scaling) as i32;

        self.health -= actual_dmg;

        if self.health <= 0 {
            self.health = 0;
            self.is_alive = false;
            println!(
                "{}{} has been defeated!{}",
                CYAN,
                self.get_troop_name(),
                RESET
            );
            return;
        }

        println!(
            "{} took {}{}{} damage!{}",
            self.get_troop_name(),
            RED,
            actual_dmg,
            RESET,
            RESET
        );
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.get_health() > self.get_max_health() {
            self.health = self.max_health;
        }
        println!(
            "{0} healed: {1}{amount}{1} health!{2}",
            self.get_troop_name(),
            BRIGHT_GREEN,
            RESET
        );
    }
    pub fn display_troops(&self) {
        println!("{}: ", self.get_troop_name());
        println!("Health: {}", self.get_health());
        println!("Type: {}", self.get_element_to_string());
    }
}
