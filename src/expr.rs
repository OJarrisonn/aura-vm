enum Expr {
    Literal(Literal),
    Sequence(Vec<Expr>)
}

enum Literal {
    Number(f64),
    String(String),
    Bool(bool)
}

impl Expr {
    pub fn eval(self) {
        match self {
            Expr::Literal(lit) => match lit {
                Literal::Number(n) => println!("{n}"),
                Literal::String(s) => println!("{s}"),
                Literal::Bool(b) => println!("{b}"),
            },
            Expr::Sequence(seq) => seq.into_iter().for_each(Expr::eval)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_simple_expression() {
        Expr::Literal(Literal::Number(8.75)).eval();
        Expr::Literal(Literal::String("Hello World!".into())).eval();
        Expr::Literal(Literal::Bool(true)).eval();
    }
}