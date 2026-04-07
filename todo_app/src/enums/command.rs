pub enum Command {
    Add(String),
    List,
    Complete(usize),
    Edit(usize, String),
    Delete(usize),
    Clear,
    Help,
    Grep {
        keyword: String,
        ignore_case: bool,
        show_line_number: bool,
    },
}
