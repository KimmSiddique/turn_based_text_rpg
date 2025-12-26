// List of ANSI color codes that will be used to provide color in the terminal
pub const RESET: &str = "\x1b[0m"; // Used for resetting color back to original
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const CYAN: &str = "\x1b[36m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";

// Unit tests to ensure that colors are working properly
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_tests() {
        println!("{0}Health: {0} {1}", RED, RESET);
        println!("Sky color: {0}BLUE {0} {1}", BLUE, RESET);
        println!(
            "Player name: {0}Jack{0}{1} Hitpoints left: {2}100 {2} {1}",
            CYAN, RESET, RED
        );

        assert!(true);
    }
}
