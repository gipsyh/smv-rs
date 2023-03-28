use crate::{
    ast::{Expr, Infix, Prefix},
    token::{
        and_tag, becomes_tag, boolean_tag, colon_tag, conditional_tag, define_tag, iff_tag,
        imply_tag, init_tag, latch_var_tag, lparen_tag, module_tag, next_tag, not_tag, or_tag,
        rparen_tag, semicolon_tag, trans_tag, Token, Tokens,
    },
    Define, Latch, SMV,
};
use nom::{
    branch::alt,
    bytes::complete::take,
    error::{Error, ErrorKind},
    error_position,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    PLowest,
    PAnd,
    POr,
    PImply,
    PIff,
}

fn parse_infix_op(input: Tokens) -> IResult<Tokens, (Precedence, Infix)> {
    let (input, op) = alt((and_tag, or_tag, imply_tag, iff_tag))(input)?;
    assert!(op.tok.len() == 1);
    Ok((
        input,
        match op.tok[0] {
            Token::And => (Precedence::PAnd, Infix::And),
            Token::Or => (Precedence::POr, Infix::Or),
            Token::Imply => (Precedence::PImply, Infix::Imply),
            Token::Iff => (Precedence::PIff, Infix::Iff),
            _ => panic!(),
        },
    ))
}

fn parse_ident(input: Tokens) -> IResult<Tokens, String> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok[0].clone() {
            Token::Ident(name) => Ok((i1, name)),
            _ => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn parse_ident_expr(input: Tokens) -> IResult<Tokens, Expr> {
    parse_ident(input).map(|(input, ident)| (input, Expr::Ident(ident)))
}

fn parse_paren_expr(input: Tokens) -> IResult<Tokens, Expr> {
    delimited(lparen_tag, parse_expr, rparen_tag)(input)
}

fn parse_prefix_expr(input: Tokens) -> IResult<Tokens, Expr> {
    let (i1, t1) = alt((not_tag, next_tag))(input)?;
    assert!(!t1.tok.is_empty());
    let (i2, e) = parse_atom_expr(i1)?;
    match t1.tok[0] {
        Token::Not => Ok((i2, Expr::PrefixExpr(Prefix::Not, Box::new(e)))),
        Token::Next => Ok((i2, Expr::PrefixExpr(Prefix::Next, Box::new(e)))),
        _ => Err(nom::Err::Error(error_position!(input, ErrorKind::Tag))),
    }
}

fn parse_conditional_expr(input: Tokens) -> IResult<Tokens, Expr> {
    let (i1, (cond, _, yes, _, no)) = tuple((
        parse_atom_expr,
        conditional_tag,
        parse_atom_expr,
        colon_tag,
        parse_atom_expr,
    ))(input)?;
    Ok((
        i1,
        Expr::ConditionalExpr {
            cond: Box::new(cond),
            yes: Box::new(yes),
            no: Box::new(no),
        },
    ))
}

fn parse_atom_expr(input: Tokens) -> IResult<Tokens, Expr> {
    alt((parse_ident_expr, parse_paren_expr, parse_prefix_expr))(input)
}

fn parse_infix_expr(
    input: Tokens,
    left: Expr,
    op: Infix,
    precedence: Precedence,
) -> IResult<Tokens, Expr> {
    let (input, right) = parse_pratt_expr(input, precedence)?;
    Ok((input, Expr::InfixExpr(op, Box::new(left), Box::new(right))))
}

fn go_parse_pratt_expr(input: Tokens, precedence: Precedence, left: Expr) -> IResult<Tokens, Expr> {
    match parse_infix_op(input) {
        Ok((i1, (peek_precedence, op))) if precedence < peek_precedence => {
            let (i2, left2) = parse_infix_expr(i1, left, op, peek_precedence)?;
            go_parse_pratt_expr(i2, precedence, left2)
        }
        _ => Ok((input, left)),
    }
}

fn parse_pratt_expr(input: Tokens, precedence: Precedence) -> IResult<Tokens, Expr> {
    dbg!(input);
    let (i1, left) = parse_atom_expr(input)?;
    go_parse_pratt_expr(i1, precedence, left)
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Expr> {
    parse_pratt_expr(input, Precedence::PLowest)
}

fn parse_define(input: Tokens) -> IResult<Tokens, Define> {
    let (i1, (ident, _, expr, _)) =
        tuple((parse_ident, becomes_tag, parse_expr, semicolon_tag))(input)?;
    Ok((i1, Define { ident, expr }))
}

fn parse_defines(input: Tokens) -> IResult<Tokens, SMV> {
    let (i1, _) = define_tag(input)?;
    many0(parse_define)(i1).map(|(tokens, defines)| {
        (
            tokens,
            SMV {
                defines,
                ..Default::default()
            },
        )
    })
}

fn parse_latch(input: Tokens) -> IResult<Tokens, Latch> {
    let (i1, (ident, _, _, _)) =
        tuple((parse_ident, colon_tag, boolean_tag, semicolon_tag))(input)?;
    Ok((i1, Latch { ident }))
}

fn parse_latchs(input: Tokens) -> IResult<Tokens, SMV> {
    let (i1, _) = latch_var_tag(input)?;
    many0(parse_latch)(i1).map(|(tokens, latchs)| {
        (
            tokens,
            SMV {
                latchs,
                ..Default::default()
            },
        )
    })
}

fn parse_inits(input: Tokens) -> IResult<Tokens, SMV> {
    let (i1, _) = init_tag(input)?;
    many0(parse_expr)(i1).map(|(input, inits)| {
        (
            input,
            SMV {
                inits,
                ..Default::default()
            },
        )
    })
}

fn parse_trans(input: Tokens) -> IResult<Tokens, SMV> {
    let (i1, _) = trans_tag(input)?;
    many0(parse_expr)(i1).map(|(input, trans)| {
        (
            input,
            SMV {
                trans,
                ..Default::default()
            },
        )
    })
}

pub fn parse_tokens(input: Tokens) -> Result<SMV, nom::Err<nom::error::Error<Tokens<'_>>>> {
    let (input, _) = module_tag(input)?;
    let (input, ident) = parse_ident(input)?;
    if ident != "main".to_string() {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
    }
    let (input, smvs) = many0(alt((parse_inits, parse_latchs, parse_defines, parse_trans)))(input)?;
    // assert!(input.tok.is_empty());
    dbg!(input);
    let smv = smvs.into_iter().fold(SMV::default(), |sum, smv| sum + smv);
    dbg!(smv);
    todo!()
}
