use crate::troop::Troop;

pub struct BattlePoints {
    battle_points: i32,
}

impl BattlePoints {
    fn new() -> Self {
        Self {
            battle_points: 1
        }
    }
    pub fn get_bp(&self) -> &i32 {
        &self.battle_points
    }
    pub fn use_bp(&mut self, bp: i32) -> bool {
        if self.battle_points >= bp {
            self.battle_points -= bp;
            return true;
        }
        return false;
    }
    pub fn gain_bp(&mut self, bp: i32) {
        self.battle_points += bp;
    }
}
pub struct Player {
    player_name: String,
    pub troops: Vec<Troop>,
    pub battle_points: BattlePoints,
}
impl Player {
    pub fn new() -> Self {
        Self {
            player_name: String::new(),
            troops: Vec::new(),
            battle_points: BattlePoints::new(),
        }
    }
    pub fn get_player_name(&self) -> &str {
        &self.player_name
    }
    pub fn set_player_name(&mut self, player_name: String) {
        self.player_name = player_name;
    }
    pub fn display_troops(&self) {
        for troop in self.troops.iter() {
            troop.display_troops();
        }
    }
    pub fn add_troop(&mut self, troop: Troop) {
        self.troops.push(troop);
    }
    pub fn get_player_vec_length(&self) -> usize {
        self.troops.len()
    }
}
