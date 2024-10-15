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