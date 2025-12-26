use crate::troop::Troop;

pub struct Player {
    player_name: String,
    troops: Vec<Troop>,
    battle_points: i32,
}
impl Player {
    pub fn new(player_name: String) -> Self {
        Self {
            player_name,
            troops: Vec::new(),
            battle_points: 1,
        }
    }
    pub fn get_player_name(&self) -> &str {
        &self.player_name
    }
    pub fn get_bp(&self) -> &i32 {
        &self.battle_points
    }
    pub fn gain_bp(&mut self, bp: i32) {
        self.battle_points += bp;
    }
    pub fn use_bp(&mut self, bp: i32) -> bool {
        if self.battle_points >= bp {
            self.battle_points -= bp;
            return true;
        }
        return false;
    }
    pub fn display_troops(&self) {
        for troop in self.troops.iter() {
            troop.display();
        }
    }
}
