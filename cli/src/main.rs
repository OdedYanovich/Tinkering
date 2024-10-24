use core::{
    state_machine::{new_game_state, GameState, Mod},
    Display,
};

struct Cli(String);
impl Display for Cli {
    fn display(s: &str) {
        println!("{s}");
    }
    fn new() -> Self {
        Cli(String::new())
    }
    fn before_action_read(&mut self) {
        std::io::stdin().read_line(&mut self.0).unwrap();
    }
    fn action_read(&self) -> char {
        self.0.chars()
    }
}
struct Location;
impl Mod for Location {
    fn state_identity<'s>() -> &'s str {
        "you're in the dungeon,\nchoose wisely\n"
    }
    fn information_display() {
        print!("");
    }
    fn action_length() -> u8 {
        1
    }
    fn new() -> Self {
        Location
    }
}
struct Credit;
impl Mod for Credit {
    fn state_identity<'s>() -> &'s str {
        "Credits\n"
    }
    fn information_display() {
        println!();
    }
    fn action_length() -> u8 {
        1
    }
    fn new() -> Self {
        Credit
    }
}
struct Options;
impl Mod for Options {
    fn state_identity<'s>() -> &'s str {
        "you're in the options menu"
    }
    fn information_display() {
        println!();
    }
    fn action_length() -> u8 {
        1
    }
    fn new() -> Self {
        Options
    }
}
struct Encounter;
impl Mod for Encounter {
    fn state_identity<'s>() -> &'s str {
        "you're in an encounter,\nAct!"
    }
    fn information_display() {
        println!();
    }
    fn action_length() -> u8 {
        1
    }
    fn new() -> Self {
        Encounter
    }
}

fn main() {
    let mut current_game_mod = new_game_state::<Cli, Location, Credit, Options, Encounter>();
    let action_button = 'a';
    current_game_mod.run(action_button);
}
