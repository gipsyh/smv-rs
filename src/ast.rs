#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    Not,
    Next,
    LtlGlobally,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    And,
    Or,
    Imply,
    Iff,
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    PrefixExpr(Prefix, Box<Expr>),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
    ConditionalExpr {
        cond: Box<Expr>,
        yes: Box<Expr>,
        no: Box<Expr>,
    },
}
