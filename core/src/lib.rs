/// A mandatory step of input processing.
trait Action {
    /// a ∈ Action.
    /// p, b ∈ button Press.
    /// a*b = a contain b.
    /// ∀a∀p∀b(a*p∧a*b ⟹ p≠b).
    /// [[#double-press|Why?]].
    fn add(&mut self, press: char);
    fn clear(&mut self);
    /// p = relevant button press, m = max size.
    /// ~x = x is an Action that impacts the game.
    /// ∀p(|p| > m ⟹ ~p) [[#short-sequence|Why one press?]] [[##cenacle-action|Why cancelling?]].
    fn len(&self) -> usize;
}
///Required implementation for each state.
///Sounds implying that players are tinkering with a machine
pub trait Audio {
    /// Play a sound in response to a button press
    fn press();
    /// Play a sound in response to a completed action.
    /// Different sounds are necessary for cancelled, failed and successful Actions
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
    fn new() -> Self;
}
pub trait Options {
    fn volume();
    ///Potential reword
    fn color_scheme();
    ///Potential reword
    fn font();
}
pub mod state_machine {
    use crate::{Action, Display};
    #[derive(PartialEq)]
    pub enum GameMods {
        Dungeon,
        Credit,
        Option,
        Encounter,
    }
    // For all game state to implement
    // trait GameState {
    //     fn action_length() -> u8;
    //     fn optional() {}
    // }
    pub struct GameState<D: Display> {
        current_mod: GameMods,
        display_tool: D,
    }
    impl<D: Display> GameState<D> {
        pub fn new() -> Self {
            D::dungeon_identity();
            Self {
                current_mod: GameMods::Dungeon,
                display_tool: D::new(),
            }
        }
        ///Run the game
        pub fn run(&mut self, _action_button: char) {
            // loop{}
            D::dungeon_information(
                "1) Start an encounter\n2) Select an Option\n3) Credits\n4) Exit the game",
            );
        }
    }
    impl GameMods {
        fn transition<A: Action, D: Display>(&mut self, _action: A) {
            D::dungeon_identity();
            match *self {
                Self::Dungeon => {} // Transitions are between Dungeon and the rest
                _ => *self = Self::Dungeon,
            }
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
    use crate::Action;
    pub struct CharSet(std::collections::HashSet<char>);
    impl CharSet {
        fn new() -> Self {
            Self(std::collections::HashSet::new())
        }
    }
    impl Action for CharSet {
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
/// Encounters will be selected randomly from the set.
mod layer_set {
    /// Growth in difficulty will be done in 2 ways:
    /// New and harder encounters will be added to the set.
    /// Easier encounters will be removed from the set
    trait CaterpillarEscalation {}
}
#[allow(dead_code)]
mod tinkering_sequels {
    use crate::Action;
    ///Introducing graphs with sequences
    mod tinkering2 {
        ///Iterate over a set of action.
        /// The possibility to start in a line that is connected to the loop is less interesting.
        struct Order;
    }
    ///Extending graphs with conditions
    mod tinkering3 {
        ///An alternative path to the order that is tied to a condition
        struct Link;
        ///The condition (at list for now)
        struct ActionSubset;
    }
    ///Effected by actions and commands independently and defining progress
    struct Environment;
    mod multiplayer {
        ///In 1 mod, players attempting to gain the largest score under restrictions.
        struct Score;
        ///Let's players compar each other's scores
        struct Instagram;
        /// Limited time event that for multiplayer
        struct TimedEncounter;
        ///Price/reminder for outstanding scores
        struct Badge;
        ///2 players sharing an environment with one's actions serving as the other's commands
        struct Fight;
    }
    ///More complicated incentive system then 2 hp bars.
    /// Probably effect conditions.
    struct Progression;
    ///Will be the first to modify an action
    trait ActionStateMachine {
        ///Internal state maps player's action to the action that will make an impact
        fn pick<A: Action>(&mut self, action: A) -> A;
    }
    ///Makes encounters easier
    struct Item;
    mod graphics {
        ///Without the need to make the effect of an action immediate,
        /// animators are getting a lot of freedom.
        struct Buffer;
    }
}
