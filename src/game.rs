use serde::{Deserialize, Serialize};

use crate::fire_troops::blazefang::Blazefang;
use crate::fire_troops::ignivore::Ignivore;
use crate::fire_troops::pyrradyn::Pyrradyn;
use crate::player::Player;
use crate::rock_troops::boulderbash::Boulderbash;
use crate::rock_troops::gravulon::Gravulon;
use crate::rock_troops::terranox::Terranox;
use crate::troop::Troop;
use crate::troop_stats::{TroopMoves, TroopStats};
use crate::utility::utility_functions::*;
use crate::utility::{color::*, utility_functions::*};
use crate::water_troops::aquashock::Aquashock;
use crate::water_troops::glacivern::Glacivern;
use crate::water_troops::torrendor::Torrendor;
use crate::{fire_troops::*, rock_troops::*, water_troops::*};

#[derive(Serialize, Deserialize)]
struct GameStats {
    total_battles: i32,
    total_rounds_played: i32,
    player1_wins: i32,
    player2_wins: i32,
    draws: i32,
}

impl GameStats {
    fn init() -> Self {
        Self {
            total_battles: 0,
            total_rounds_played: 0,
            player1_wins: 0,
            player2_wins: 0,
            draws: 0,
        }
    }
    fn get_total_battles(&self) -> &i32 {
        &self.total_battles
    }
    fn get_total_rounds_played(&self) -> &i32 {
        &self.total_rounds_played
    }
    fn get_player1_wins(&self) -> &i32 {
        &self.player1_wins
    }
    fn get_player2_wins(&self) -> &i32 {
        &self.player2_wins
    }
    fn get_player_draws(&self) -> &i32 {
        &self.player2_wins
    }
}

struct Game {
    winner: String,
    player1: Player,
    player2: Player,
    round_num: i32,

    is_player1_turn: bool,
    is_running: bool,
    game_stats: GameStats,
}

impl Game {
    fn init(
        winner: String,
        round_num: i32,
        is_player1_turn: bool,
        is_running: bool,
        game_stats: GameStats,
    ) -> Self {
        Self {
            winner: String::new(),
            player1: Player::new(),
            player2: Player::new(),
            round_num: 1,
            is_player1_turn: true,
            is_running: true,
            game_stats: GameStats::init(),
        }
    }
    fn start_screen() {
        print!("{}", CYAN);
        println!(
            r"
        
    //     __        _______ _     ____ ___  __  __ _____                                
    //     \ \      / / ____| |   / ___/ _ \|  \/  | ____|                               
    //      \ \ /\ / /|  _| | |  | |  | | | | |\/| |  _|                                 
    //       \ V  V / | |___| |__| |__| |_| | |  | | |___                                
    //        \_/\_/  |_____|_____\____\___/|_|  |_|_____|                               
    //                     |_   _/ _  |                                                   
    //                       | || | | |                                                  
    //                       | || |_| |                                                  
    //      ____  ____   ____|_| \___/    _  _____ _____ _     _____     ____ ___ __  __ 
    //     |  _ \|  _ \ / ___|   | __ )  / \|_   _|_   _| |   | ____|   / ___|_ _|  \/  |
    //     | |_) | |_) | |  _    |  _ \ / _ \ | |   | | | |   |  _|     \___ \| || |\/| |
    //     |  _ <|  __/| |_| |   | |_) / ___ \| |   | | | |___| |___     ___) | || |  | |
    //     |_| \_\_|    \____|   |____/_/   \_\_|   |_| |_____|_____|   |____/___|_|  |_|
    //                         "
        );
        println!("{}", RESET);
    }
    fn display_troops(&self) {
        println!("{MAGENTA}=====================================================");
        println!("               AVAILABLE TROOPS LIST");
        println!("====================================================={RESET}");

        println!(
            "{:<3} {:<15} {:<5} {}Fire{RESET}",
            "1.", "Blazefang", "Type:", RED
        );
        println!(
            "{:<3} {:<15} {:<5} {}Fire{RESET}",
            "2.", "Ignivore", "Type:", RED
        );
        println!(
            "{:<3} {:<15} {:<5} {}Fire{RESET}",
            "3.", "Pyrradyn", "Type:", RED
        );

        println!(
            "{:<3} {:<15} {:<5} {}Water{RESET}",
            "4.", "Aquashock", "Type:", BLUE
        );
        println!(
            "{:<3} {:<15} {:<5} {}Water{RESET}",
            "5.", "Glacivern", "Type:", BLUE
        );
        println!(
            "{:<3} {:<15} {:<5} {}Water{RESET}",
            "6.", "Torrendor", "Type:", BLUE
        );

        println!(
            "{:<3} {:<15} {:<5} {}Rock{RESET}",
            "7.", "Boulderbash", "Type:", YELLOW
        );
        println!(
            "{:<3} {:<15} {:<5} {}Rock{RESET}",
            "8.", "Terranox", "Type:", YELLOW
        );
        println!(
            "{:<3} {:<15} {:<5} {}Rock{RESET}",
            "9.", "Gravulon", "Type:", YELLOW
        );
    }

    fn init_players(&mut self) {
        let player1_name = take_string_input("Enter name for Player 1");
        let player2_name: String = take_string_input("Enter name for Player 2");

        self.player1.set_player_name(player1_name);
        self.player2.set_player_name(player2_name);

        println!();
        thread_sleep_for_ms(1000);
        clear_screen();

        // Pick troops:
        self.pick_troops();
        clear_screen();

        println!("{}'s team: ", self.player1.get_player_name());
        self.player1.display_troops();
        thread_sleep_for_ms(2000);
        clear_screen();
        println!("{}'s team: ", self.player2.get_player_name());
        thread_sleep_for_ms(2000);
        clear_screen();

        println!("Battle will now commence!");
        thread_sleep_for_ms(2000);
    }
    fn pick_troops(&mut self) {
        self.display_troops();

        println!("{RED}<Enter corresponding number next to troop to add to your army>{RESET}");

        for i in 0..3 {
            // -------- Player 1 --------
            let choice_p1 = loop {
                println!(
                    "{} choose troop number {}:",
                    self.player1.get_player_name(),
                    i + 1
                );

                let input = take_int_input("");
                if (1..=9).contains(&input) {
                    break input;
                }

                println!("{RED}Invalid choice, enter a number between 1 and 9{RESET}");
            };

            add_troop(choice_p1, &mut self.player1);

            // -------- Player 2 --------
            let choice_p2 = loop {
                println!(
                    "{} choose troop number {}:",
                    self.player2.get_player_name(),
                    i + 1
                );

                let input = take_int_input("");
                if (1..=9).contains(&input) {
                    break input;
                }

                println!("{RED}Invalid choice, enter a number between 1 and 9{RESET}");
            };

            add_troop(choice_p2, &mut self.player2);

            thread_sleep_for_ms(1100);
            clear_screen();
            self.display_troops();
        }
    }

    fn update_round(&self, player1: &mut Player, player2: &mut Player) {
        todo!("Will implement this later...");
    }
    fn display_choices(&self) {
        todo!("Will implement this later...");
    }
    fn display_battle(&self) {
        todo!("Will implement this later...");
    }
    fn take_player_input(&mut self, choice: i32) {
        let active_troop_p1 = match self.player1.troops.first_mut() {
            Some(t) => t,
            None => return,
        };
        let active_troop_p2 = match self.player2.troops.first_mut() {
            Some(t) => t,
            None => return,
        };

        if self.is_player1_turn {
            match choice {
                1 => active_troop_p1.use_move_1(
                    &mut self.player1.battle_points.get_bp_mut(),
                    active_troop_p2.stats_mut(),
                ),
                2 => active_troop_p1.use_move_2(
                    &mut self.player1.battle_points.get_bp_mut(),
                    active_troop_p2.stats_mut(),
                ),
                3 => active_troop_p1.use_move_3(
                    &mut self.player1.battle_points.get_bp_mut(),
                    active_troop_p2.stats_mut(),
                ),
                _ => unreachable!(),
            }
        } else {
            match choice {
                1 => active_troop_p2.use_move_1(
                    &mut self.player2.battle_points.get_bp_mut(),
                    active_troop_p1.stats_mut(),
                ),
                2 => active_troop_p2.use_move_1(
                    &mut self.player2.battle_points.get_bp_mut(),
                    active_troop_p1.stats_mut(),
                ),
                3 => active_troop_p2.use_move_1(
                    &mut self.player2.battle_points.get_bp_mut(),
                    active_troop_p1.stats_mut(),
                ),
                _ => unreachable!(),
            }
        }
    }

    fn remove_defeated_troops(player: &mut Player) {
        todo!("Will implement this later...");
    }
    fn save_game_stats(&self) {
        todo!("Will implement this later...");
    }
    fn load_game_stats(&self) {
        todo!("Will implement this later...");
    }
    fn display_game_stats(&self) {
        println!("\n----------STATS----------");
        println!(
            "Total Battles Played: {}",
            self.game_stats.get_total_battles()
        );
        println!(
            "Total Rounds Played: {}",
            self.game_stats.get_total_rounds_played()
        );
        println!("Player 1 victories: {}", self.game_stats.get_player1_wins());
        println!("Player 2 victories: {}", self.game_stats.get_player2_wins());
        println!("Draws: {}\n", self.game_stats.get_player_draws());
    }
    fn battle(&self) {
        todo!("Will implement this later...");
    }
}
