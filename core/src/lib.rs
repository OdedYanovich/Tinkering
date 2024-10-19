pub mod state_machine {
    pub trait Action {
        fn new() -> Self;
        fn add(&mut self, press: char);
        fn clear(&mut self);
        fn len(&self) -> usize;
    }
    ///1 for each state
    trait Audio {
        /// Play a sound in response to a button press
        fn press();
        /// Play a sound in response to a completed action
        fn action(results: bool);
        /// Play a sound in the background
        fn background();
    }
    ///1 for each state
    pub trait Display {
        /// Show a visual that gives the player information that is relevant to his current information
        fn dungeon_information(s: &str);
        /// Show a visual that lets the player know in which state he is
        fn dungeon_identity();
    }
    // trait GameState {
    //     fn action_length() -> u8;
    //     fn optional() {}
    // }
    pub fn run<T: Display>(_t: &T, _action_button: char) {
        T::dungeon_information(
            "1) Start an encounter\n2) Select an Option\n3) Credits\n4) Exit the game",
        );
    }
    #[derive(PartialEq)]
    pub enum States {
        Dungeon,
        Credit,
        Option,
        Encounter,
    }
    impl States {
        pub fn new<D: Display>() -> Self {
            D::dungeon_identity();
            Self::Dungeon
        }
        fn transition<A: Action, D: Display>(&mut self, _action: A) {
            D::dungeon_identity();
            match *self {
                Self::Dungeon => {} // Transitions are between Dungeon and the rest
                _ => *self = Self::Dungeon,
            }
            // Credit
            // Options
            // Encounter
        }
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
        fn len(&self) -> usize {
            self.0.len()
        }
    }
}
