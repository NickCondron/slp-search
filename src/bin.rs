pub mod args;
pub mod regex;

use std::{fs, io};
use clap::Parser;
use peppi::model::{
    game::{ Player, Game },
    enums::{
        character::External,
        stage::Stage,
    },
};
use peppi::serde::de;

use slp_search::MatchedPlayers;
use args::{Cli, Commands, Filter, SharedArgs};
use crate::regex::*;

fn main() {
    let cli = Cli::parse();

    for slp in &cli.shared_args().replays {
        let path = slp.as_path();
        if !path.is_file() {
            eprintln!("{} does not exist", path.to_string_lossy());
            continue;
        }

        // Attempt to parse games without frames
        let f = fs::File::open(path).unwrap();
        let mut buf = io::BufReader::new(f);
        let opts = Some(de::Opts { skip_frames: true, debug_dir: None });
        let game = peppi::game(&mut buf, opts.as_ref(), None);
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
}

fn do_filter(game: &Game, filter_args: &Filter) -> bool {
    filter_args.stage.as_ref()
        .and_then(|s| Stage::try_match(s))
        .map(|s| s == game.start.stage)
        .unwrap_or(true)
}

fn match_players(
    game: &Game,
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
    player: &Player,
    ignorecase: bool,
    char: &Option<String>,
    name: &Option<String>,
    code: &Option<String>,
) -> bool {
    char.as_ref()
        .and_then(|c| External::try_match(c))
        .map(|c| c == player.character)
        .unwrap_or(true) &&
    match (name, code) {
        (None, None) => true,
        (name, code) => {
            let np = player.netplay.as_ref().expect("No Netplay data found");
            name.as_ref().map(|n| {
                ignorecase && equals_ignorecase(n, &np.name) || n == &np.name
            }).unwrap_or(true) &&
            code.as_ref().map(|c| {
                ignorecase && equals_ignorecase(c, &np.code) || c == &np.code
            }).unwrap_or(true)
        }
    }
}

fn equals_ignorecase(s1: &str, s2: &str) -> bool {
    s1.to_ascii_uppercase() == s2.to_ascii_uppercase()
}
