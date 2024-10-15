pub trait GameState {
    fn visuals(s: &str);
    fn background_music(){}
    fn background_sound(){}
    fn action_length() -> u8;
    fn optional() {}
    fn event(&self) {
        Self::visuals("your under attack!");
    }
}
pub trait Dungeon: GameState {}
pub trait Credit: GameState {}
pub trait Options: GameState {}
pub trait Encounter: GameState {}