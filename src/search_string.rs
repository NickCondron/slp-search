use super::lib::{Action, FrameGap, Percent, MRange, Player, Token};
use nom::error::{ErrorKind, ParseError};
use nom::Err::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, char, multispace1};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use peppi::model::enums::action_state::{Common, State};

#[derive(Debug, PartialEq)]
pub enum SearchParseError<I> {
    Nom(I, ErrorKind),
    IllegalRange,
}

impl<I> ParseError<I> for SearchParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        SearchParseError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

type MResult<I, O> = IResult<I, O, SearchParseError<I>>;

fn mrange_single(i: &[u8]) -> MResult<&[u8], MRange> {
    let (i, (_, num)) = tuple((char('='), complete::u32))(i)?;
    Ok((i, MRange {
            start: num,
            end: num,
        }
    ))
}

fn mrange_multiple(i: &[u8]) -> MResult<&[u8], MRange> {
    let (i, (low, _, high)) = tuple((complete::u32, tag(".."), complete::u32))(i)?;
    Ok((i, MRange {
            start: low,
            end: high,
        }
    ))
}

fn mrange_capped(i: &[u8]) -> MResult<&[u8], MRange> {
    let (i, (_, high)) = tuple((tag(".."), complete::u32))(i)?;
    Ok((i, MRange {
            start: 0,
            end: high,
        }
    ))
}

fn mrange(i: &[u8]) -> MResult<&[u8], MRange> {
    let (i, range) = alt((mrange_single, mrange_multiple, mrange_capped))(i)?;
    if range.start > range.end {
        Err(Error(SearchParseError::IllegalRange))
    } else {
        Ok((i, range))
    }
}

fn frame_gap(i: &[u8]) -> MResult<&[u8], FrameGap> {
    map(tuple((tag("fg"), mrange)), |(_, range)| {
        FrameGap {
            range: range,
        }
    })(i)
}

fn player(i: &[u8]) -> MResult<&[u8], Player> {
    alt((
        map(char('p'), |_| Player::Player),
        map(char('o'), |_| Player::Opponent),
    ))(i)
}

fn action(i: &[u8]) -> MResult<&[u8], Action> {
    map(tuple((char('.'), player, complete::u16)), |(_, player, id)| {
        Action {
            player: player,
            state: State::Common(Common(id)),
        }
    })(i)
}

fn percent(i: &[u8]) -> MResult<&[u8], Percent> {
    map(tuple((char('%'), player, mrange)), |(_, player, range)| {
        Percent {
            player: player,
            range: range,
        }
    })(i)
}

fn token(i: &[u8]) -> MResult<&[u8], Token> {
    alt((
        map(frame_gap, |t| Token::FrameGap(t)),
        map(percent, |t| Token::Percent(t)),
        map(action, |t| Token::Action(t)),
    ))(i)
}

pub fn search_string(i: &[u8]) -> MResult<&[u8], Vec<Token>> {
    separated_list1(multispace1, token)(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::*;

    #[test]
    fn test_frame_gap() {
        let s1 = b"fg3..7";
        let s2 = b"fg..30";
        let s3 = b"fg=2";

        let fg1 = FrameGap { range: MRange { start: 3, end: 7 } };
        let fg2 = FrameGap { range: MRange { start: 0, end: 30 } };
        let fg3 = FrameGap { range: MRange { start: 2, end: 2 } };

        let r1 = frame_gap(s1).unwrap().1;
        let r2 = frame_gap(s2).unwrap().1;
        let r3 = frame_gap(s3).unwrap().1;

        assert_eq!(r1, fg1);
        assert_eq!(r2, fg2);
        assert_eq!(r3, fg3);
    }
    #[test]
    fn test_percent() {
        let s1 = b"%p3..7";
        let s2 = b"%o..30";
        let s3 = b"%p=2";

        let p1 = Percent { player: Player::Player, range: MRange { start: 3, end: 7 } };
        let p2 = Percent { player: Player::Opponent, range: MRange { start: 0, end: 30 } };
        let p3 = Percent { player: Player::Player, range: MRange { start: 2, end: 2 } };

        let r1 = percent(s1).unwrap().1;
        let r2 = percent(s2).unwrap().1;
        let r3 = percent(s3).unwrap().1;

        assert_eq!(r1, p1);
        assert_eq!(r2, p2);
        assert_eq!(r3, p3);
    }

    #[test]
    fn test_token_count() {
        let s =b".p14   fg=1\n%p50..200";
        assert_eq!(search_string(s).unwrap().1.len(), 3);
    }
}
