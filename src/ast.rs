#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    Not,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    And,
    Or,
}

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    PrefixExpr(Prefix, Box<Expr>),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    ConditionalExpr {
        cond: Box<Expr>,
        yes: Box<Expr>,
        no: Box<Expr>,
    },
}
