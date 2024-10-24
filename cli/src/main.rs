use core::{
    state_machine::{new_game_state, Mod},
    Action, Display,
};
struct CliOut(std::vec::IntoIter<char>);
impl CliOut {
    fn fill_presses_iterator() -> Self {
        let mut t = String::new();
        std::io::stdin().read_line(&mut t).unwrap();
        Self(t.chars().collect::<Vec<_>>().into_iter())
    }
}
impl Display for CliOut {
    fn display(s: &str) {
        println!("{s}");
    }
    fn new() -> Self {
        Self::fill_presses_iterator()
    }
    fn before_action_read(&mut self) {
        *self = Self::fill_presses_iterator();
    }
    fn action_read(&mut self) -> char {
        self.0.next().unwrap()
    }
}
struct CliIn(std::vec::IntoIter<char>);
impl CliIn {
    fn fill_presses_iterator() -> Self {
        let mut t = String::new();
        std::io::stdin().read_line(&mut t).unwrap();
        Self(t.chars().collect::<Vec<_>>().into_iter())
    }
}
impl Action for CliIn {
    fn new() -> Self {
        Self::fill_presses_iterator()
    }
}
struct Location;
impl Mod for Location {
    fn state_identity<D: Display>() {
        D::display("you're in the dungeon,\nchoose wisely\n")
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
    fn state_identity<D: Display>() {
        D::display("Credits\n");
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
    fn state_identity<D: Display>() {
        D::display("you're in the options menu");
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
    fn state_identity<D: Display>() {
        D::display("you're in an encounter,\nAct!");
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
    let mut current_game_mod = new_game_state::<CliOut, Location, Credit, Options, Encounter>();
    let action_button = 'a';
    current_game_mod.run(action_button);
}
