#[derive(Debug)]
pub enum Token {
    Word { contents: String },
    Whitespace { contents: String },
    Var { name: String },
    BlockOpen,
    BlockClose,
    String,
    BracketOpen,
    BracketClose
}