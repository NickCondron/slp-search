use peppi::model::enums::character;
use peppi::model::primitives::Port;

pub enum MatchedPlayers {
    OneWay(Port, Port),
    Both(Port, Port),
    NoMatch,
}

pub struct Context {
    pub p_character: character::Internal,
    pub p_port: Port,
    pub o_character: character::Internal,
    pub o_port: Port,
}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum Player {
    Player,
    Opponent,
}

#[derive(Debug,PartialEq,Eq)]
pub struct MRange {
    pub start: u32,
    pub end: u32,
}

impl MRange {
    pub fn contains(&self, num: u32) -> bool {
        self.start <= num && num <= self.end
    }
}

pub type AnchorGroup = Vec<FrameAnchor>;

#[derive(Debug)]
pub struct Query {
    pub first_anchor: AnchorGroup,
    pub remaining: Vec<(FrameGap, AnchorGroup)>,
}

#[derive(Debug,PartialEq,Eq)]
pub enum FrameAnchor {
    Action(Action),
    Percent(Percent),
}

impl FrameAnchor {
    pub fn player(&self) -> Player {
        match self {
            FrameAnchor::Action(a) => a.player,
            FrameAnchor::Percent(p) => p.player,
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
pub struct FrameGap {
    pub range: MRange,
    //pub skip_hitlag: bool,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Action {
    pub player: Player,
    pub state_id: u16,
    //pub age: f32,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Percent {
    pub player: Player,
    pub range: MRange,
}

pub struct Match {
    pub frame_start: i32,
    pub frame_end: i32,
}
