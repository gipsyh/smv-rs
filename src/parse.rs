use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0, multispace1, newline},
    multi::many0,
    sequence::tuple,
    IResult,
};


pub fn parse_empty_line(input: (&str, i32)) -> IResult<&str, ()> {
    let input = input.0;
    tuple((many0(char(' ')), newline))(input).map(|res| (res.0, ()))
}

pub fn parse_module_main(input: &str) -> IResult<&str, ()> {
    tuple((
        multispace0,
        tag("MODULE"),
        multispace1,
        tag("main"),
        many0(char(' ')),
        newline,
    ))(input)
    .map(|res| (res.0, ()))
}
// pub fn parse_define()
