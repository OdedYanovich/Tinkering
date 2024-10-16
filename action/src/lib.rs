use encounter::Action;
pub struct CharSet(std::collections::HashSet<char>);
impl Action for CharSet {
    fn new() -> Self {
        Self(std::collections::HashSet::new())
    }
    fn add(&mut self, press: char) {
        self.0.insert(press);
    }
    fn clear(&mut self){
        self.0.clear();
    }
}