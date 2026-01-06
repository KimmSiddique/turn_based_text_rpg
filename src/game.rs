use serde::{Deserialize, Serialize};

use crate::player::Player;

use crate::utility::color::*;
use crate::utility::utility_functions::*;

#[derive(Serialize, Deserialize)]
pub struct GameStats {
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
        &self.draws
    }
    fn increment_player1_wins(&mut self) {
        self.player1_wins += 1;
    }
    fn increment_player2_wins(&mut self) {
        self.player2_wins += 1;
    }
    fn increment_draws(&mut self) {
        self.draws += 1;
    }
    fn increment_total_rounds_played(&mut self) {
        self.total_rounds_played += 1;
    }
    fn increment_total_battles(&mut self) {
        self.total_battles += 1;
    }
}

pub struct Game {
    winner: String,
    player1: Player,
    player2: Player,
    round_num: i32,

    is_player1_turn: bool,
    is_running: bool,
    game_stats: GameStats,
}

impl Game {
    pub fn init() -> Self {
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

    fn update_round(&mut self) {
        remove_defeated_troops(&mut self.player1);
        remove_defeated_troops(&mut self.player2);

        if self.player1.troops.is_empty() && self.player2.troops.is_empty() {
            self.winner = "Draw".to_string();
            self.game_stats.increment_draws();
            self.is_running = false;
        } else if self.player1.troops.is_empty() {
            self.winner = self.player2.get_player_name().to_string();
            self.game_stats.increment_player2_wins();
            self.is_running = false;
        } else if self.player2.troops.is_empty() {
            self.winner = self.player1.get_player_name().to_string();
            self.game_stats.increment_player1_wins();
            self.is_running = false;
        }
        // BP increases by 1 each round, from round 5 onwards it increases by 2
        if self.round_num >= 5 {
            self.player1.battle_points.gain_bp(2);
            self.player2.battle_points.gain_bp(2);
        } else {
            self.player1.battle_points.gain_bp(1);
            self.player2.battle_points.gain_bp(1);
        }

        self.round_num += 1;
        self.game_stats.increment_total_rounds_played();
    }
    fn display_choices(&mut self) {
        let active_troop_p1 = match self.player1.troops.first() {
            Some(t) => t,
            None => return,
        };
        let active_troop_p2 = match self.player2.troops.first() {
            Some(t) => t,
            None => return,
        };

        if self.is_player1_turn {
            active_troop_p1.display_moves();
        } else {
            active_troop_p2.display_moves();
        }

        loop {
            let choice = take_int_input("\nEnter your choice (1, 2 or 3)");

            if (1..=3).contains(&choice) {
                self.take_player_input(choice);
                break; // exit loop ONLY after valid input
            } else {
                println!("{RED}Invalid choice, please enter 1, 2 or 3{RESET}");
            }
        }
    }

    fn display_battle(&self) {
        {
            // Header
            println!(
                "{YELLOW}================ ROUND {} ================{RESET}",
                self.round_num
            );

            // Player headers
            let p1_name = format!("{} (P1)", self.player1.get_player_name());
            let p2_name = format!("{} (P2)", self.player2.get_player_name());
            println!("{:<30} | {}", p1_name, p2_name);

            // Active troop names (front of vector)
            let p1_active = self
                .player1
                .troops
                .first()
                .map(|t| t.get_troop_name())
                .unwrap_or("<none>");
            let p2_active = self
                .player2
                .troops
                .first()
                .map(|t| t.get_troop_name())
                .unwrap_or("<none>");

            println!(
                "{:<30} | {}",
                format!("Active: {}", p1_active),
                format!("Active: {}", p2_active)
            );

            // Battle points (BattlePoints struct inside Player)
            println!(
                "{:<30} | {}",
                format!("BP: {}", self.player1.battle_points.get_bp()),
                format!("BP: {}", self.player2.battle_points.get_bp())
            );

            // Troops remaining
            println!(
                "{:<30} | {}",
                format!("Troops remaining: {}", self.player1.troops.len()),
                format!("Troops remaining: {}", self.player2.troops.len())
            );

            println!("{CYAN}-----------------------------------------------------------{RESET}");

            // Troop list header
            println!("{:<30} | {}", "Player 1 Troops", "Player 2 Troops");

            // List troops side-by-side
            let max_size = self.player1.troops.len().max(self.player2.troops.len());
            for i in 0..max_size {
                let left = if i < self.player1.troops.len() {
                    let t = &self.player1.troops[i];
                    format!(
                        "{} ({}/{})",
                        t.get_troop_name(),
                        t.get_health(),
                        t.get_max_health()
                    )
                } else {
                    String::new()
                };

                let right = if i < self.player2.troops.len() {
                    let t = &self.player2.troops[i];
                    format!(
                        "{} ({}/{})",
                        t.get_troop_name(),
                        t.get_health(),
                        t.get_max_health()
                    )
                } else {
                    String::new()
                };

                println!("{:<30} | {}", left, right);
            }

            println!("{CYAN}-----------------------------------------------------------{RESET}");
        }
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
            self.is_player1_turn = false;
        } else {
            match choice {
                1 => active_troop_p2.use_move_1(
                    &mut self.player2.battle_points.get_bp_mut(),
                    active_troop_p1.stats_mut(),
                ),
                2 => active_troop_p2.use_move_2(
                    &mut self.player2.battle_points.get_bp_mut(),
                    active_troop_p1.stats_mut(),
                ),
                3 => active_troop_p2.use_move_3(
                    &mut self.player2.battle_points.get_bp_mut(),
                    active_troop_p1.stats_mut(),
                ),
                _ => unreachable!(),
            }
            self.is_player1_turn = true;
        }
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
    fn battle(&mut self) {
        clear_screen();
        println!("{YELLOW}=== BATTLE PHASE ==={RESET}\n");

        self.round_num = 1;

        while self.is_running {
            clear_screen();
            self.display_battle();

            if self.is_player1_turn {
                println!("\n{CYAN}{}'s Turn!{RESET}", self.player1.get_player_name());
            } else {
                println!("\n{CYAN}{}'s Turn!{RESET}", self.player2.get_player_name());
            }

            self.display_choices();

            // Keep update_round as a method on Game; it can mutate both players and set winner/is_running.
            self.update_round();

            thread_sleep_for_ms(2500);
        }

        clear_screen();
        println!("{GREEN}\n=== BATTLE OVER ===\n{RESET}");
        println!("{YELLOW}Winner: {}{RESET}\n", self.winner);
        self.game_stats.increment_total_battles();
    }
    pub fn run_game(&mut self) {
        // One-time startup
        Game::start_screen();
        thread_sleep_for_ms(900);

        // Load saved stats if available (non-fatal if missing)
        self.load_stats_into_game();
        thread_sleep_for_ms(600);

        // Main program loop (menu)
        loop {
            clear_screen();
            println!("{MAGENTA}==============================={RESET}");
            println!("{CYAN}         MAIN MENU{RESET}");
            println!("{MAGENTA}==============================={RESET}");
            println!("1) Start New Battle");
            println!("2) Instructions");
            println!("3) View Stats");
            println!("4) Quit");
            println!();

            let choice = take_int_input("Choose an option (1-4)");

            match choice {
                1 => {
                    // ----- Reset per-battle state -----
                    self.winner = String::new();
                    self.round_num = 1;
                    self.is_player1_turn = true;
                    self.is_running = true;

                    // If Player::new() makes empty troop lists etc, re-init players here.
                    // This avoids old troop data leaking between battles.
                    self.player1 = Player::new();
                    self.player2 = Player::new();

                    clear_screen();

                    // Setup players (names + troop selection)
                    self.init_players();

                    // Run battle loop (this will set is_running = false when battle ends)
                    self.battle();

                    // After battle: show stats & persist
                    self.display_game_stats();
                    save_game_stats(&self.game_stats);

                    wait_for_enter();
                }

                2 => {
                    clear_screen();
                    display_instructions_txt(); // already does wait_for_enter() internally
                }

                3 => {
                    clear_screen();
                    self.display_game_stats();
                    wait_for_enter();
                }

                4 => {
                    clear_screen();
                    println!("{GREEN}Thanks for playing!{RESET}");
                    // Save stats on exit too (safe)
                    save_game_stats(&self.game_stats);
                    break;
                }

                _ => {
                    eprintln!("{RED}Invalid option. Please choose 1-4.{RESET}");
                    thread_sleep_for_ms(1200);
                }
            }
        }
    }
    fn load_stats_into_game(&mut self) {
        match load_game_stats() {
            Some(stats) => self.game_stats = stats,
            None => eprintln!("{RED}Game-stats file not found{RESET}"),
        }
    }
}
