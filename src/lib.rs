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

#[derive(Default, Debug)]
pub struct SMV {
    pub defines: Vec<Define>,
    pub latchs: Vec<Latch>,
    pub inits: Vec<Expr>,
    pub trans: Vec<Expr>,
}

impl SMV {
    fn parse(input: &str) -> Result<(), nom::Err<nom::error::Error<&str>>> {
        let tokens = lex_tokens(input)?;
        let tokens = Tokens::new(&tokens);
        dbg!(tokens);
        parse_tokens(tokens);
        Ok(())
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> io::Result<Self> {
        let s = read_to_string(file)?;
        Self::parse(&s).unwrap();
        Ok(Self {
            ..Default::default()
        })
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
        self.inits.extend(rhs.inits);
        self.trans.extend(rhs.trans);
    }
}
