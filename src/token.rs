#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Int(i32),
    Plus,
    LeftParen,
    RightParen,
}
