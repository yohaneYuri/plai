use crate::ast::Expr;

pub fn calculate(expr: Expr) -> i32 {
    match expr {
        Expr::Num(num) => num,
        Expr::Plus(l, r) => calculate(*l) + calculate(*r),
        Expr::Parened(expr) => calculate(*expr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_num() {
        let num = Expr::Num(1);
        assert_eq!(1, calculate(num));
    }

    #[test]
    fn plain_additional_expr() {
        let expr = Expr::Plus(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)));
        assert_eq!(3, calculate(expr));
    }

    #[test]
    fn parened_additional_expr() {
        let parened = Expr::Plus(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)));
        let expr = Expr::Plus(Box::new(Expr::Num(1)), Box::new(parened));
        assert_eq!(6, calculate(expr));
    }
}
