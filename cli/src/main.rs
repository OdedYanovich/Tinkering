use core::tinkering_api::{run, Display, States};

struct Cli;
impl Display for Cli {
    fn dungeon_identity() {
        println!("you're in the Dungeon,\nAct!");
    }
    fn dungeon_information(s: &str) {
        println!("{s}");
    }
}
fn main() {
    let cli = Cli;
    let state = States::new::<Cli>();
    // loop{}
    let action_button = 'a';
    run(&cli, action_button);
}
