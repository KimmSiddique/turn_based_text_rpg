use crate::fire_troops::{blazefang::Blazefang, ignivore::Ignivore, pyrradyn::Pyrradyn};
use crate::rock_troops::{boulderbash::Boulderbash, gravulon::Gravulon, terranox::Terranox};
use crate::water_troops::{aquashock::Aquashock, glacivern::Glacivern, torrendor::Torrendor};
use crate::player::Player;
use crate::troop_stats::{TroopMoves, TroopStats};

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

impl Troop {
    pub fn display(&self) {
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
    pub fn use_move_1(&mut self, player: &mut Player, target: &mut TroopStats) {
         match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.move_1(player, target),
            Troop::Ignivore(i) => i.move_1(player, target),
            Troop::Pyrradyn(p) => p.move_1(player, target),

            // Water-troops:
            Troop::Aquashock(a) => a.move_1(player, target),
            Troop::Glacivern(g) => g.move_1(player, target),
            Troop::Torrendor(t) => t.move_1(player, target),

            // Rock-troops:
            Troop::Boulderbash(b) => b.move_1(player, target),
            Troop::Gravulon(g) => g.move_1(player, target),
            Troop::Terranox(t) => t.move_1(player, target),
        }
    }
     pub fn use_move_2(&mut self, player: &mut Player, target: &mut TroopStats) {
         match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.move_2(player, target),
            Troop::Ignivore(i) => i.move_2(player, target),
            Troop::Pyrradyn(p) => p.move_2(player, target),

            // Water-troops:
            Troop::Aquashock(a) => a.move_2(player, target),
            Troop::Glacivern(g) => g.move_2(player, target),
            Troop::Torrendor(t) => t.move_2(player, target),

            // Rock-troops:
            Troop::Boulderbash(b) => b.move_2(player, target),
            Troop::Gravulon(g) => g.move_2(player, target),
            Troop::Terranox(t) => t.move_2(player, target),
        }
    }
     pub fn use_move_3(&mut self, player: &mut Player, target: &mut TroopStats) {
         match self {
            // Fire-troops:
            Troop::Blazefang(b) => b.move_3(player, target),
            Troop::Ignivore(i) => i.move_3(player, target),
            Troop::Pyrradyn(p) => p.move_3(player, target),

            // Water-troops:
            Troop::Aquashock(a) => a.move_3(player, target),
            Troop::Glacivern(g) => g.move_3(player, target),
            Troop::Torrendor(t) => t.move_3(player, target),

            // Rock-troops:
            Troop::Boulderbash(b) => b.move_3(player, target),
            Troop::Gravulon(g) => g.move_3(player, target),
            Troop::Terranox(t) => t.move_3(player, target),
        }
    }
}
