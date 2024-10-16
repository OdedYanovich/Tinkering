pub trait GameState {
    fn visuals(s: &str);
    fn background_music() {}
    fn background_sound() {}
    fn action_length() -> u8;
    fn optional() {}
    fn event(&self) {
        Self::visuals("Your under attack!");
    }
}
pub trait Dungeon: GameState {}
// pub trait Credit: GameState + Transition {}
// pub trait Options: GameState + Transition {}
// pub trait Encounter: GameState + Transition {}
pub trait Transition<A: Dungeon, B: GameState> {
    fn from_dungeon();
    fn to_dungeon();
}
