use core::{
    state_machine::{GameState, Mod},
    Display,
};

struct Cli;
impl Display for Cli {
    fn display(s: &str) {
        println!("{s}");
    }
    fn new() -> Self {
        Cli
    }
}
struct Location;
impl Mod for Location {
    fn identity<'s>() -> &'s str {
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
    fn identity<'s>() -> &'s str {
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
    fn identity<'s>() -> &'s str {
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
    fn identity<'s>() -> &'s str {
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
    let mut current_game_mod = GameState::<Cli, Location, Credit, Options, Encounter>::new();
    let action_button = 'a';
    current_game_mod.run(action_button);
}
