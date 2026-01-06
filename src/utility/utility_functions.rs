use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::fire_troops::{blazefang::Blazefang, ignivore::Ignivore, pyrradyn::Pyrradyn};
use crate::game::GameStats;
use crate::player::Player;
use crate::rock_troops::{boulderbash::Boulderbash, gravulon::Gravulon, terranox::Terranox};
use crate::troop::Troop;
use crate::utility::color::{GREEN, RESET};
use crate::water_troops::{aquashock::Aquashock, glacivern::Glacivern, torrendor::Torrendor};

pub fn thread_sleep_for_ms(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    use std::io::Write;
    let _ = std::io::stdout().flush();
}

pub fn take_int_input(message: &str) -> i32 {
    loop {
        let mut user_input = String::new();

        print!("{message}: ");
        io::stdout().flush().expect("Failed to flush stdout");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => match user_input.trim().parse::<i32>() {
                Ok(input) => return input,
                Err(error) => {
                    println!("Invalid integer value: {error}");
                }
            },
            Err(error) => {
                println!("Error reading input: {error}");
            }
        }
    }
}

pub fn take_string_input(message: &str) -> String {
    let input = loop {
        let mut user_input = String::new();

        print!("{message}: ");
        io::stdout().flush().expect("Failed to flush stdout");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => break user_input,
            Err(error) => println!("Error reading input: {error}"),
        }
    };
    input.trim().to_string()
}

pub fn add_troop(choice: i32, player: &mut Player) {
    match choice {
        // Fire-troops
        1 => player.add_troop(Troop::Blazefang(Blazefang::new())),
        2 => player.add_troop(Troop::Ignivore(Ignivore::new())),
        3 => player.add_troop(Troop::Pyrradyn(Pyrradyn::new())),
        // Water-troops
        4 => player.add_troop(Troop::Aquashock(Aquashock::new())),
        5 => player.add_troop(Troop::Glacivern(Glacivern::new())),
        6 => player.add_troop(Troop::Torrendor(Torrendor::new())),
        // Rock-troops
        7 => player.add_troop(Troop::Boulderbash(Boulderbash::new())),
        8 => player.add_troop(Troop::Terranox(Terranox::new())),
        9 => player.add_troop(Troop::Gravulon(Gravulon::new())),
        _ => eprintln!("Invalid choice"),
    }
}

pub fn wait_for_enter() {
    let mut input = String::new();
    println!("{GREEN}\nPress Enter to continue...{RESET}");
    let _ = io::stdin().read_line(&mut input);
}

pub fn remove_defeated_troops(player: &mut Player) {
    player.troops.retain(|troop| troop.get_is_alive());
}

pub fn display_instructions_txt() {
    match fs::read_to_string("assets/instructions.txt") {
        Ok(contents) => {
            println!("{contents}");
            wait_for_enter();
        }
        Err(error) => {
            eprintln!("Could not read instructions.txt: {error}");
        }
    }
}

pub fn save_game_stats(game_stats: &GameStats) {
    let json = match serde_json::to_string_pretty(game_stats) {
        Ok(j) => j,
        Err(error) => {
            eprintln!("Failed to save Game-stats: {error}");
            return;
        }
    };

    match std::fs::write("assets/stats.json", json) {
        Ok(_) => {
            println!("Game-stats saved successfully!");
        }
        Err(error) => {
            eprintln!("Failed to save Game-stats: {error}");
        }
    }
}

pub fn load_game_stats() -> Option<GameStats> {
    let contents = match std::fs::read_to_string("assets/stats.json") {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to load Game-stats file: {error}");
            return None;
        }
    };

    let game_stats = match serde_json::from_str(&contents) {
        Ok(string) => string,
        Err(error) => {
            eprintln!("Failed to parse Game-stats JSON: {error}");
            return None;
        }
    };

    Some(game_stats)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_int_input() {
        let number = take_int_input("Enter a number");
        println!("The number you typed was: {number}");
        assert!(true);
    }

    #[test]
    fn test_take_string_input() {
        let string = take_string_input("Enter a word");
        println!("You entered: {string}");
        assert!(true);
    }
}
