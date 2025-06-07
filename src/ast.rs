#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Num(i32),
    Plus(Box<Expr>, Box<Expr>),
}
