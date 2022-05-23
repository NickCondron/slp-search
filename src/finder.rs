use super::lib::*;
use peppi::model::frame::{Frame, Post};
use peppi::model::enums::action_state::State;

pub fn find(context: &Context, query: &Query, frames: &Vec<Frame<2>>) -> Match {

    let mut start_frame: i32 = -123;
    for frame in frames {
        if match_anchor_group(&query.first_anchor, context, frame) {
            start_frame = frame.index;
        }
    }

    Match { frame_start: start_frame, frame_end: 0 }
}

fn match_anchor_group(anchor_group: &AnchorGroup, context: &Context, frame: &Frame<2>) -> bool {
    anchor_group.iter().all(|anchor| match_anchor(&anchor, context, frame))
}

fn match_anchor(anchor: &FrameAnchor, context: &Context, frame: &Frame<2>) -> bool {
    let port_idx = match (anchor.player(), context.p_port < context.o_port) {
        (Player::Player, true) => 0,
        (Player::Opponent, false) => 0,
        _ => 1,
    };

    let post = &frame.ports[port_idx].leader.post;
    //let state  = action_state::State::from(10,Internal::MARTH);
    match anchor {
        FrameAnchor::Action(a) => State::from(a.state_id, post.character) == post.state,
        FrameAnchor::Percent(p) => p.range.contains(post.damage.floor() as u32),
    }
}
