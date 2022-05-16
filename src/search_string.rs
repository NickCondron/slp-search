use super::lib::{Player, Token, Action, FrameGap, Percent};
use nom::IResult;
use nom::branch::alt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::character::complete::{self, multispace1, char};
use nom::bytes::complete::{tag};
use nom::combinator::{map, opt};
use std::ops::Range;

fn range_single(i: &[u8]) -> IResult<&[u8], Range<u32>> {
    let (i, (_, num)) = tuple((char('='), complete::u32))(i)?;
    Ok((i, Range { start: num, end: num }))
}
fn range_multiple(i: &[u8]) -> IResult<&[u8], Range<u32>> {
    let (i, (low, _, high)) = tuple((complete::u32, tag(".."), complete::u32))(i)?;
    Ok((i, Range { start: low, end: high }))
}
fn range(i: &[u8]) -> IResult<&[u8], Range<u32>> {
    alt((range_single, range_multiple))(i)
}

fn frame_gap(i: &[u8]) -> IResult<&[u8], Token> {
    let (i, _) = tag("fg")(i)?;
    let (i, range) = range(i)?;
    Ok((i, Token::FrameGap(FrameGap { range: range })))
}

fn player(i: &[u8]) -> IResult<&[u8], Player> {
    alt((
        map(char('p'), |_| Player::Player),
        map(char('o'), |_| Player::Opponent),
    ))(i)
}


fn action(i: &[u8]) -> IResult<&[u8], Token> {
    todo!()
}

fn percent(i: &[u8]) -> IResult<&[u8], Token> {
    map(tuple((char('%'), player, range)),
        |(_, player, range)|
        Token::Percent(Percent { player: player, range: range }))(i)
}

pub fn parse_string(search_string: &[u8]) -> IResult<&[u8], Vec<Token>> {
    separated_list1(multispace1, alt((
        frame_gap,
        action,
        percent,
    )))(search_string)
}
