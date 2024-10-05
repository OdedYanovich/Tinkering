pub trait Button {}

trait Action: Iterator<Item: Button> {
    fn set_length(&self);
    fn keep<A: Action, P: RelevantButtonPress>(a: &mut A, press: P);
}

trait Challenge: IntoIterator<Item = char> {}

enum Instruction {
    Here,
    NotHere,
    Mix,
}
trait Command {}

fn comparison<A: Action, C: Challenge>(action: impl Action, challenge: impl Command) {}

trait RelevantButtonPress {}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
