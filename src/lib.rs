use peppi::model::enums::character;
use peppi::model::enums::action_state::State;
use peppi::model::primitives::Port;

pub enum MatchedPlayers {
    OneWay(Port, Port),
    Both(Port, Port),
    NoMatch,
}

pub struct Context {
    pub player_character: character::Internal,
    pub player_port: Port,
    pub opponent_character: character::Internal,
    pub opponent_port: Port,
}

#[derive(Debug,PartialEq,Eq)]
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
    fn contains(&self, num: u32) -> bool {
        self.start <= num && num <= self.end
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum Token {
    FrameGap(FrameGap),
    Action(Action),
    Percent(Percent),
}

#[derive(Debug,PartialEq,Eq)]
pub struct FrameGap {
    pub range: MRange,
    //pub skip_hitlag: bool,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Action {
    pub player: Player,
    pub state: State,
    //pub age: f32,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Percent {
    pub player: Player,
    pub range: MRange,
}

pub struct MatchResult {
    pub context: Context,
    pub frame_start: i32,
    pub frame_end: i32,
}
