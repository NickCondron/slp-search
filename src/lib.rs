use peppi::model::enums::character;
use peppi::model::enums::action_state::State;
use peppi::model::primitives::Port;
use std::ops::Range;

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

pub enum Player {
    Player,
    Opponent,
}

pub enum Token {
    FrameGap(FrameGap),
    Action(Action),
    Percent(Percent),
}

pub struct FrameGap {
    pub range: Range<u32>,
    //pub skip_hitlag: bool,
}

pub struct Action {
    pub player: Player,
    pub state: State,
    pub age: f32,
}

pub struct Percent {
    pub player: Player,
    pub range: Range<u32>,
}

pub struct MatchResult {
    pub context: Context,
    pub frame_start: i32,
    pub frame_end: i32,
}
