mod define;
mod expr;
mod lexer;
mod parse;
mod symbol;
mod var;
mod ltl;

use lexer::lex_tokens;
use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::{alpha1, digit1, multispace0, multispace1},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};
use parse::parse_empty_line;
use std::{fs::read_to_string, io, path::Path};

use crate::parse::parse_module_main;

pub struct SMV {}

impl SMV {
    fn parse(input: &str) -> IResult<(), ()> {
        // let mut parser = tuple((alpha1, digit1, alpha1));
        // delimited(multispace0, tag("MODULE"), multispace0)(input)
        // delimited(multispace0, tag("main"), multispace0)(input)
        // let (input, _) = many0(alt((parse_comment, parse_empty_line)))((input, 1))?;
        let tokens = lex_tokens(input).unwrap();
        dbg!(tokens);
        Ok(((), ()))
        // let (input, _) = parse_module_main(input)?;
        // let (input, _) = many0(alt((parse_comment, parse_empty_line)))((input, 1))?;
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> io::Result<Self> {
        let s = read_to_string(file)?;
        dbg!(&s);
        dbg!(Self::parse(&s).unwrap());
        Ok(Self {})
    }
}
