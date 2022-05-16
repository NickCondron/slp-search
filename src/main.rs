pub mod args;
pub mod lib;

use lib::MatchedPlayers;
use args::{Cli, Commands, Filter, SharedArgs};
use clap::Parser;
use peppi::model::enums::character::External;
use peppi::model::enums::stage::Stage;
use peppi::model::*;
use peppi::serde::de;
use std::{fs, io};


fn main() {
    let cli = Cli::parse();

    for slp in cli.replays() {
        let path = slp.as_path();
        if !path.is_file() {
            eprintln!("{} does not exist", path.to_string_lossy());
            continue;
        }

        // Attempt to parse games without frames
        let f = fs::File::open(path).unwrap();
        let mut buf = io::BufReader::new(f);
        let skip_frames = Some(de::Opts { skip_frames: true });
        let game = peppi::game(&mut buf, skip_frames, None);
        if let Err(e) = game {
            eprintln!("{}", e);
            continue;
        }
        let game = game.unwrap();

        let args: &SharedArgs = cli.shared_args();
        let players = match_players(
            &game,
            args.ignorecase,
            &args.pchar,
            &args.pname,
            &args.pcode,
            &args.ochar,
            &args.oname,
            &args.ocode,
        );
        if let MatchedPlayers::NoMatch = players {
            continue;
        }

        match cli.command {
            Commands::Filter(ref filter_args) => {
                if do_filter(&game, &filter_args) {
                    println!("{:#?}", path.to_string_lossy());
                }
            },
            Commands::Search(ref _search_args) => {
                todo!();
            },
        }
    }

    //let state  = State::from("ATTACK_11");//Falco.REFLECTOR_AIR_CHANGE_DIRECTION;
    //let state  = action_state::State::from(10,Internal::MARTH);
    //let state: State  = State::Common(Common(10));
    //let state = Common::WAIT;
}

fn do_filter(game: &game::Game, filter_args: &Filter) -> bool {
    if let Some(stage) = &filter_args.stage {
        match_stage(&stage, &game.start.stage)
    } else {
        true
    }
}

fn match_players(
    game: &game::Game,
    ignorecase: bool,
    pchar: &Option<String>,
    pname: &Option<String>,
    pcode: &Option<String>,
    ochar: &Option<String>,
    oname: &Option<String>,
    ocode: &Option<String>
) -> MatchedPlayers {
    let players = &game.start.players;

    if players.len() != 2 {
        return MatchedPlayers::NoMatch;
    }

    let p1 = &players[0];
    let p2 = &players[1];

    let player_matches1 = match_player(p1, ignorecase, pchar, pname, pcode);
    let player_matches2 = match_player(p2, ignorecase, pchar, pname, pcode);
    let oppon_matches1 = match_player(p1, ignorecase, ochar, oname, ocode);
    let oppon_matches2 = match_player(p2, ignorecase, ochar, oname, ocode);

    // println!("{}, {}, {}, {}",
    //     player_matches1,
    //     player_matches2,
    //     oppon_matches1,
    //     oppon_matches2,
    // );

    match (
        player_matches1,
        player_matches2,
        oppon_matches1,
        oppon_matches2,
    ) {
        (true, true, true, true) => MatchedPlayers::Both(p1.port, p2.port),
        (true, _, _, true) => MatchedPlayers::OneWay(p1.port, p2.port),
        (_, true, true, _) => MatchedPlayers::OneWay(p2.port, p1.port),
        _ => MatchedPlayers::NoMatch,
    }
}

fn match_player(
    player: &game::Player,
    ignorecase: bool,
    car: &Option<String>,
    name: &Option<String>,
    code: &Option<String>,
) -> bool {
    let np = player.netplay.as_ref().expect("No Netplay data found");

    (match car {
        None => true,
        Some(c) => match_character(&c, &player.character)
    }) &&
    (match (name, code) {
        (None, None) => true,
        (name, code) => (match name {
            None => true,
            Some(n) => ignorecase && equals_ignorecase(n, &np.name) || n == &np.name,
        }) && (match code {
            None => true,
            Some(c) => ignorecase && equals_ignorecase(c, &np.code) || c == &np.code,
        })
    })
}

fn equals_ignorecase(s1: &str, s2: &str) -> bool {
    s1.to_ascii_uppercase() == s2.to_ascii_uppercase()
}

fn match_stage(stage: &str, id: &Stage) -> bool {
    let formatted = stage.replace(" ", "_").to_ascii_uppercase();
    let formatstr = formatted.as_str();
    if let Ok(stageid) = Stage::try_from(formatstr) {
        stageid == *id
    } else {
        false
    }
}

fn match_character(character: &str, id: &External) -> bool {
    let formatted = character.replace(" ", "_").to_ascii_uppercase();
    let formatstr = formatted.as_str();
    if let Ok(charid) = External::try_from(formatstr) {
        charid == *id
    } else {
        false
    }
}
