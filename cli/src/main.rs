use dungeon::{Encounter, GameState};

struct Cli;
impl GameState for Cli {
    fn visuals(s: &str) {
        println!("{s}");
    }
    fn action_length() -> u8 {
        1
    }
}
impl Encounter for Cli {}

fn main() {
    let t = Cli;
    GameState::event(&t);
}
