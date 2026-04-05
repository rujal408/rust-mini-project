pub enum Command {
    Add(String),
    List,
    Complete(usize),
    Edit(usize, String),
    Delete(usize),
    Clear,
}
