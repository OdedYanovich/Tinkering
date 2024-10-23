use core::{state_machine::GameState, Display};

struct Cli;
impl Display for Cli {
    fn dungeon_identity() {
        println!("you're in the Dungeon,\nAct!");
    }
    fn dungeon_information(s: &str) {
        println!("{s}");
    }
    fn new() -> Self {
        Cli
    }
}
fn main() {
    let mut current_game_mod = GameState::<Cli>::new();
    let action_button = 'a';
    current_game_mod.run(action_button);
}
