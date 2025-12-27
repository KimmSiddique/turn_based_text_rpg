use crate::fire_troops::{blazefang::Blazefang, ignivore::Ignivore, pyrradyn::Pyrradyn};
use crate::player::BattlePoints;
use crate::rock_troops::{boulderbash::Boulderbash, gravulon::Gravulon, terranox::Terranox};
use crate::troop_stats::{TroopMoves, TroopStats};
use crate::water_troops::{aquashock::Aquashock, glacivern::Glacivern, torrendor::Torrendor};

pub enum Troop {
    // Fire-troops:
    Blazefang(Blazefang),
    Ignivore(Ignivore),
    Pyrradyn(Pyrradyn),

    // Water troops:
    Aquashock(Aquashock),
    Glacivern(Glacivern),
    Torrendor(Torrendor),

    // Rock troops:
    Boulderbash(Boulderbash),
    Gravulon(Gravulon),
    Terranox(Terranox),
}

// Method for display + moves
impl Troop {
    pub fn display_troops(&self) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.stats.display_troops(),
            Troop::Ignivore(i) => i.stats.display_troops(),
            Troop::Pyrradyn(p) => p.stats.display_troops(),

            // Water-troops:
            Troop::Aquashock(a) => a.stats.display_troops(),
            Troop::Glacivern(g) => g.stats.display_troops(),
            Troop::Torrendor(t) => t.stats.display_troops(),

            // Rock-troops:
            Troop::Boulderbash(b) => b.stats.display_troops(),
            Troop::Gravulon(g) => g.stats.display_troops(),
            Troop::Terranox(t) => t.stats.display_troops(),
        }
    }
    pub fn use_move_1(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.move_1(player_bp, target),
            Troop::Ignivore(i) => i.move_1(player_bp, target),
            Troop::Pyrradyn(p) => p.move_1(player_bp, target),

            // Water-troops:
            Troop::Aquashock(a) => a.move_1(player_bp, target),
            Troop::Glacivern(g) => g.move_1(player_bp, target),
            Troop::Torrendor(t) => t.move_1(player_bp, target),

            // Rock-troops:
            Troop::Boulderbash(b) => b.move_1(player_bp, target),
            Troop::Gravulon(g) => g.move_1(player_bp, target),
            Troop::Terranox(t) => t.move_1(player_bp, target),
        }
    }
    pub fn use_move_2(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.move_2(player_bp, target),
            Troop::Ignivore(i) => i.move_2(player_bp, target),
            Troop::Pyrradyn(p) => p.move_2(player_bp, target),

            // Water-troops:
            Troop::Aquashock(a) => a.move_2(player_bp, target),
            Troop::Glacivern(g) => g.move_2(player_bp, target),
            Troop::Torrendor(t) => t.move_2(player_bp, target),

            // Rock-troops:
            Troop::Boulderbash(b) => b.move_2(player_bp, target),
            Troop::Gravulon(g) => g.move_2(player_bp, target),
            Troop::Terranox(t) => t.move_2(player_bp, target),
        }
    }
    pub fn use_move_3(&mut self, player_bp: &mut BattlePoints, target: &mut TroopStats) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.move_3(player_bp, target),
            Troop::Ignivore(i) => i.move_3(player_bp, target),
            Troop::Pyrradyn(p) => p.move_3(player_bp, target),

            // Water-troops:
            Troop::Aquashock(a) => a.move_3(player_bp, target),
            Troop::Glacivern(g) => g.move_3(player_bp, target),
            Troop::Torrendor(t) => t.move_3(player_bp, target),

            // Rock-troops:
            Troop::Boulderbash(b) => b.move_3(player_bp, target),
            Troop::Gravulon(g) => g.move_3(player_bp, target),
            Troop::Terranox(t) => t.move_3(player_bp, target),
        }
    }
}

// Implementation for heal, take_damage and display_moves and stats_mut():
impl Troop {
    pub fn take_dmg(&mut self, dmg: i32) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.stats.take_dmg(dmg),
            Troop::Ignivore(i) => i.stats.take_dmg(dmg),
            Troop::Pyrradyn(p) => p.stats.take_dmg(dmg),

            // Water-troops:
            Troop::Aquashock(a) => a.stats.take_dmg(dmg),
            Troop::Glacivern(g) => g.stats.take_dmg(dmg),
            Troop::Torrendor(t) => t.stats.take_dmg(dmg),

            // Rock Troops:
            Troop::Boulderbash(b) => b.stats.take_dmg(dmg),
            Troop::Gravulon(g) => g.stats.take_dmg(dmg),
            Troop::Terranox(t) => t.stats.take_dmg(dmg),
        }
    }
    pub fn heal(&mut self, amount: i32) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.stats.heal(amount),
            Troop::Ignivore(i) => i.stats.heal(amount),
            Troop::Pyrradyn(p) => p.stats.heal(amount),

            // Water-troops:
            Troop::Aquashock(a) => a.stats.heal(amount),
            Troop::Glacivern(g) => g.stats.heal(amount),
            Troop::Torrendor(t) => t.stats.heal(amount),

            // Rock Troops:
            Troop::Boulderbash(b) => b.stats.heal(amount),
            Troop::Gravulon(g) => g.stats.heal(amount),
            Troop::Terranox(t) => t.stats.heal(amount),
        }
    }
    pub fn display_moves(&self) {
        match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.stats.display_moves(),
            Troop::Ignivore(i) => i.stats.display_moves(),
            Troop::Pyrradyn(p) => p.stats.display_moves(),

            // Water-troops:
            Troop::Aquashock(a) => a.stats.display_moves(),
            Troop::Glacivern(g) => g.stats.display_moves(),
            Troop::Torrendor(t) => t.stats.display_moves(),

            // Rock-troops:
            Troop::Boulderbash(b) => b.stats.display_moves(),
            Troop::Gravulon(g) => g.stats.display_moves(),
            Troop::Terranox(t) => t.stats.display_moves(),
        }
    }
    pub fn stats_mut(&mut self) -> &mut TroopStats {
        match self {
            Troop::Blazefang(b) => &mut b.stats,
            Troop::Ignivore(i) => &mut i.stats,
            Troop::Pyrradyn(p) => &mut p.stats,

            Troop::Aquashock(a) => &mut a.stats,
            Troop::Glacivern(g) => &mut g.stats,
            Troop::Torrendor(t) => &mut t.stats,

            Troop::Boulderbash(b) => &mut b.stats,
            Troop::Gravulon(g) => &mut g.stats,
            Troop::Terranox(t) => &mut t.stats,
        }
    }
}
