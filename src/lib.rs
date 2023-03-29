mod ast;
mod lexer;
mod parser;
mod token;

use crate::{parser::parse_tokens, token::Tokens};
use ast::Expr;
use lexer::lex_tokens;
use std::{
    fs::read_to_string,
    io,
    ops::{Add, AddAssign},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Define {
    pub ident: String,
    pub expr: Expr,
    pub flatten: bool,
}

#[derive(Debug)]
pub struct Var {
    pub ident: String,
}

#[derive(Default, Debug)]
pub struct SMV {
    pub defines: Vec<Define>,
    pub vars: Vec<Var>,
    pub inits: Vec<Expr>,
    pub trans: Vec<Expr>,
    pub invariants: Vec<Expr>,
    pub fairness: Vec<Expr>,
    pub ltlspecs: Vec<Expr>,
}

impl SMV {
    fn flatten_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::LitExpr(_) => expr,
            Expr::Ident(ident) => {
                for i in 0..self.defines.len() {
                    if self.defines[i].ident == ident {
                        if self.defines[i].flatten {
                        } else {
                            self.defines[i].expr = self.flatten_expr(self.defines[i].expr.clone());
                            self.defines[i].flatten = true;
                        }
                        return self.defines[i].expr.clone();
                    }
                }
                for latch in self.vars.iter() {
                    if latch.ident == ident {
                        return Expr::Ident(ident);
                    }
                }
                panic!()
            }
            Expr::PrefixExpr(op, sub_expr) => {
                Expr::PrefixExpr(op, Box::new(self.flatten_expr(*sub_expr)))
            }
            Expr::InfixExpr(op, left, right) => Expr::InfixExpr(
                op,
                Box::new(self.flatten_expr(*left)),
                Box::new(self.flatten_expr(*right)),
            ),
            Expr::CaseExpr(mut case_expr) => {
                case_expr.branchs = case_expr
                    .branchs
                    .into_iter()
                    .map(|(x, y)| (self.flatten_expr(x), self.flatten_expr(y)))
                    .collect();
                Expr::CaseExpr(case_expr)
            }
            Expr::ConditionalExpr {
                cond: _,
                yes: _,
                no: _,
            } => todo!(),
        }
    }

    fn flatten_defines(&mut self) {
        for i in 0..self.defines.len() {
            self.flatten_expr(Expr::Ident(self.defines[i].ident.clone()));
        }
        for i in 0..self.inits.len() {
            self.inits[i] = self.flatten_expr(self.inits[i].clone());
        }
        for i in 0..self.trans.len() {
            self.trans[i] = self.flatten_expr(self.trans[i].clone());
        }
        for i in 0..self.invariants.len() {
            self.invariants[i] = self.flatten_expr(self.invariants[i].clone());
        }
        for i in 0..self.fairness.len() {
            self.fairness[i] = self.flatten_expr(self.fairness[i].clone());
        }
        for i in 0..self.ltlspecs.len() {
            self.ltlspecs[i] = self.flatten_expr(self.ltlspecs[i].clone());
        }
    }
}

impl SMV {
    fn parse(input: &str) -> Self {
        let tokens = lex_tokens(input).unwrap();
        let tokens = Tokens::new(&tokens);
        let mut smv = parse_tokens(tokens).unwrap();
        smv.flatten_defines();
        smv
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> io::Result<Self> {
        let s = read_to_string(file)?;
        Ok(Self::parse(&s))
    }
}

impl Add for SMV {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for SMV {
    fn add_assign(&mut self, rhs: Self) {
        self.defines.extend(rhs.defines);
        self.vars.extend(rhs.vars);
        self.inits.extend(rhs.inits);
        self.trans.extend(rhs.trans);
        self.invariants.extend(rhs.invariants);
        self.fairness.extend(rhs.fairness);
        self.ltlspecs.extend(rhs.ltlspecs);
    }
}
