#[derive(Default, Clone, PartialEq)]
pub struct TerminalHistory {
    pub commands: Vec<String>,
    pub outputs: Vec<String>,
}