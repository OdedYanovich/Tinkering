pub type Interface(action,audio,display){
  Action(new: fn(Int)->action, add: fn(String)->action,get_press_amount:fn(action)->Int)
  Audio(press: fn()->audio, action:fn(Bool)->audio,background:fn()->Bool)
  Display(new:fn()->display,print:fn(String)->Nil, action_read:fn())
}

pub type DisplayInterface
///1 for each state
pub trait Display {
    /// Display information that is relevant to his current situation
    fn display(s: &str);
    /// Make a new display tool
    fn new() -> Self;
    ///Lets users initialize the Display object for action read
    fn before_action_read(&mut self) {}
    ///
    fn action_read(&mut self) -> char;
}