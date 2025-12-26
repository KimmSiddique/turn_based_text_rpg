use crate::fire_troops::{blazefang::Blazefang, ignivore::Ignivore, pyrradyn::Pyrradyn};
use crate::rock_troops::{boulderbash::Boulderbash, gravulon::Gravulon, terranox::Terranox};
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
}
