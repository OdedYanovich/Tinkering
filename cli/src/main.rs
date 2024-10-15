use dungeon::{Dungeon, GameState};

struct Cli;
impl GameState for Cli {
    fn visuals(s: &str) {
        println!("{s}");
    }
    fn action_length() -> u8 {
        1
    }
}
impl Dungeon for Cli {}

fn main() {
    let t = Cli;
    GameState::event(&t);
}
