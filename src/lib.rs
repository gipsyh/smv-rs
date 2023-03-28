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

#[derive(Debug)]
pub struct Define {
    pub ident: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Latch {
    pub ident: String,
}

#[derive(Debug)]
pub struct Input {
    pub ident: String,
}

#[derive(Default, Debug)]
pub struct SMV {
    pub defines: Vec<Define>,
    pub latchs: Vec<Latch>,
    pub inputs: Vec<Input>,
    pub inits: Vec<Expr>,
    pub trans: Vec<Expr>,
    pub ltlspecs: Vec<Expr>,
}

impl SMV {
    fn parse(input: &str) -> Self {
        let tokens = lex_tokens(input).unwrap();
        let tokens = Tokens::new(&tokens);
        parse_tokens(tokens).unwrap()
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
        self.latchs.extend(rhs.latchs);
        self.inputs.extend(rhs.inputs);
        self.inits.extend(rhs.inits);
        self.trans.extend(rhs.trans);
        self.ltlspecs.extend(rhs.ltlspecs);
    }
}
