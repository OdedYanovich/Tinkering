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
    /// Display information that is relevant to his current situation
    fn display(s: &str);
    /// Make a new display tool
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
    use crate::Display;
    /// For all game state to implement
    pub trait Mod {
        /// Information presentation instruction
        fn information_display();
        /// Provide information that is unique to the state and never changes
        fn identity<'s>() -> &'s str;
        ///returns the amount of button necessary to complete the action
        fn action_length() -> u8;
        /// Make a new Mod
        fn new() -> Self;
    }
    pub trait MainState: Mod {}
    pub trait SideState: Mod {}
    #[derive(PartialEq)]
    pub enum GameMods<L: MainState, C: SideState, O: SideState, E: SideState> {
        Location(L),
        Credit(C),
        Option(O),
        Encounter(E),
    }
    impl<L: MainState, C: SideState, O: SideState, E: SideState> GameMods<L, C, O, E> {
        /// The only acceptable state change between Dungeon and the rest
        fn transition<A: crate::action::Action, D: Display>(&mut self, _action: A) {
            let t = &self;
            let mod_identity;
            *self = match t {
                Self::Location(l) => {
                    mod_identity = L::identity();
                    todo!() //Can transition to any other state
                }
                GameMods::Credit(c) => {
                    mod_identity = C::identity();
                    GameMods::Location(L::new())
                }
                GameMods::Option(o) => {
                    mod_identity = O::identity();
                    GameMods::Location(L::new())
                }
                GameMods::Encounter(e) => {
                    mod_identity = E::identity();
                    GameMods::Location(L::new())
                }
            };
            D::display(mod_identity);
        }
    }
    pub struct GameState<D: Display, L: MainState, C: SideState, O: SideState, E: SideState> {
        current_mod: GameMods<L, C, O, E>,
        display_tool: D,
    }
    impl<D: Display, L: MainState, C: SideState, O: SideState, E: SideState> GameState<D, L, C, O, E> {
        pub fn new() -> Self {
            D::display(L::identity());
            Self {
                current_mod: GameMods::<L, C, O, E>::Location(L::new()),
                display_tool: D::new(),
            }
        }
        ///Run the game
        pub fn run(&mut self, _action_button: char) {
            let location_information = [
                "1) Start an encounter",
                "2) Select an Option",
                "3) Credits",
                "4) Exit the game",
            ];
            location_information.iter().for_each(|s| {
                let t = &(*self).current_mod;
                match *t {
                    GameMods::Location(_) => {
                        L::information_display();
                    }
                    _ => {}
                }
                D::display(s);
            });
        }
    }
}
mod encounter {
    ///Define a subset of the potential permutations of the player's actions
    ///that they will be reworded for providing a permutation that is owned by that subset.
    ///Show the current and future command at the same time
    trait Command: Iterator<Item = char> + Sized {}

    fn check_action<C: Command>(action: &str, command: C) -> u8 {
        let mut result = 0;
        for (press, expected) in action.chars().zip(command) {
            if press == expected {
                result += 1;
            }
        }
        result
    }
    //Manage array of indices to a different array
    struct IndexingTool<'d, 'i> {
        data: &'d [char],
        indices: &'i [u8],
        index: u8,
    }
    impl<'d, 'i> Iterator for IndexingTool<'d, 'i> {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.index += 1;
            Some(self.data[self.indices[(self.index - 1) as usize] as usize])
        }
    }
    impl<'a, 'b> Command for IndexingTool<'a, 'b> {}
}
mod action {
    /// A mandatory step of input processing.
    pub trait Action {
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
        fn optional() {}
        //Encounter could gain an identity if they will start as soon as the player
        //press a button instead of as soon as the first action is made
    }
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
mod dungeon {
    /// Encounters will be selected randomly from the set.
    mod layer {
        pub struct Layer;
        /// Growth in difficulty will be done in 2 ways:
        /// New and harder encounters will be added to the set.
        /// Easier encounters will be removed from the set
        trait CaterpillarEscalation {
            // The first Layers are reducing enemies without randomness (2-1, 3-1, 3-2).
            // They will end with a layer that include all of them in a row.
        }
        trait ProgressThreshold {}
        trait show_encounters {
            fn layout(amount_of_encounters: u8);
            fn position(id: u8);
        }
    }
    trait LayerSelector {}

    trait MakeEncounter {
        fn from_layer(t: layer::Layer);
    }
}
#[allow(dead_code)]
mod tinkering_sequels {
    use crate::action::Action;
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
    ///A sequence of actions that players can do in order during an encounter
    /// when the current action is within the set of the current command
    /// for a reword
    trait OptionalChallenge {
        ///a = Action, c = Challenge,
        ///x - y = x removes y
        ///x + y = x exposes y
        ///@x = x just happened
        ///*x = current Command ∈ x
        ///∃c∃a(@a ∧ c + a ∧ *a ⊂ c - a)
        fn conditional_remove();
        ///c = challenge
        /// * = reword is given
        ///{ ∀c:|c| == 0 } = *
        fn conditional_reword();
    }
    ///Allows commands to require players to treat the buttons within them differently.
    ///For example,
    trait Instruction{}
}
