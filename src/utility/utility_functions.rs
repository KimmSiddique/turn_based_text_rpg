use std::io::{self, Write, stdin, stdout};
use std::thread;
use std::time::Duration;

pub fn thread_sleep_for_ms(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().expect("Failed to flush stdout");
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
    loop {
        let mut user_input = String::new();

        print!("{message}: ");
        io::stdout().flush().expect("Failed to flush stdout");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => return user_input,
            Err(error) => println!("Error reading input: {error}"),
        }
    }
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