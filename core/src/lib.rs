pub mod state_machine {
    pub trait Action {
        fn new() -> Self;
        fn add(&mut self, press: char);
        fn clear(&mut self);
    }
    pub trait Display {
        /// Show a visual
        fn show(s: &str);
        /// Play a sound
        fn play();
    }
    pub trait GameState {
        fn visuals(s: &str);
        fn background_audio() {}
        fn action_audio() {}
        fn action_length() -> u8;
        fn optional() {}
    }
    pub fn run<T: Display>(_t: &T) {
        T::show("Your in the Dungeon,\n make your choice:\n1) Start an encounter\n2) Select an Option\n3) Credits\n4) Exit the game");
    }
    pub trait CenterState: GameState {}
    // pub trait Credit: GameState + Transition {}
    // pub trait Options: GameState + Transition {}
    // pub trait Encounter: GameState + Transition {}
    trait Transition<A: CenterState, B: GameState> {
        fn from_dungeon();
        fn to_dungeon();
    }
}

mod encounter {
    struct Command;

    fn check_action(action: &str, command: IndexingTool) -> u8 {
        let mut result = 0;
        for (press, expected) in action.chars().zip(command) {
            if press == expected {
                result += 1;
            }
        }
        result
    }
    struct IndexingTool<'a, 'b> {
        data: &'a [char],
        indices: &'b [u8],
        index: u8,
    }
    impl<'a, 'b> Iterator for IndexingTool<'a, 'b> {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.index += 1;
            Some(self.data[self.indices[(self.index - 1) as usize] as usize])
        }
    }
}

mod action {
    use crate::state_machine::Action;
    pub struct CharSet(std::collections::HashSet<char>);
    impl Action for CharSet {
        fn new() -> Self {
            Self(std::collections::HashSet::new())
        }
        fn add(&mut self, press: char) {
            self.0.insert(press);
        }
        fn clear(&mut self) {
            self.0.clear();
        }
    }
}
