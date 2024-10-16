use core::state_machine::{run, Display};

struct Dungeon;
impl Display for Dungeon {
    fn show(s: &str) {
        println!("{s}");
    }
    fn play() {}
}
fn main() {
    let t = Dungeon;
    run(&t);
}
