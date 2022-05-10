use clap::Parser;
//use peppi::model::enums::action_state::{Common, State};
use peppi::model::enums::character::External;
use peppi::model::primitives::Port;
use peppi::model::*;
use std::{fs, io, path};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    // Case insensitive
    //#[clap(short,long)]
    //ignorecase: bool,
    /// Stage
    #[clap(short, long)]
    stage: Option<String>,

    /// Player character
    #[clap(long)]
    pchar: Option<String>,

    /// Player netplay name
    #[clap(long)]
    pname: Option<String>,

    /// Player netplay connect code (eg. MANG#0)
    #[clap(long)]
    pcode: Option<String>,

    /// Opponent character
    #[clap(long)]
    ochar: Option<String>,

    /// Opponent netplay name
    #[clap(long)]
    oname: Option<String>,

    /// Opponent netplay connect code (eg. MANG#0)
    #[clap(long)]
    ocode: Option<String>,

    /// Replays to search
    replays: Vec<path::PathBuf>,
}

enum MatchedPlayers {
    OneWay(Port, Port),
    Both(Port, Port),
    NoMatch(),
}
use crate::MatchedPlayers::*;

fn main() {
    let cli = Cli::parse();

    for slp in &cli.replays {
        let path = slp.as_path();
        if !path.is_file() {
            println!("{} does not exist", path.to_string_lossy());
            continue;
        }

        let f = fs::File::open(path).unwrap();
        let mut buf = io::BufReader::new(f);
        let game = peppi::game(&mut buf, None, None).unwrap();
        //println!("{:#?}", game);

        match match_players(&game, &cli) {
            MatchedPlayers::NoMatch() => {
                continue;
            }
            _ => {
                println!("{:#?}", path.to_string_lossy());
            }
        };
    }

    //let state  = State::from("ATTACK_11");//Falco.REFLECTOR_AIR_CHANGE_DIRECTION;
    //let state  = action_state::State::from(10,Internal::MARTH);
    //let state: State  = State::Common(Common(10));
    //let state = Common::WAIT;
}

fn match_players(game: &game::Game, cli: &Cli) -> MatchedPlayers {
    let players = &game.start.players;

    if players.len() != 2 {
        return NoMatch();
    }

    let p1 = &players[0];
    let p2 = &players[1];

    let player_matches1 = match_player(&p1, &cli.pchar, &cli.pname, &cli.pcode);
    let player_matches2 = match_player(&p2, &cli.pchar, &cli.pname, &cli.pcode);
    let oppon_matches1 = match_player(&p1, &cli.ochar, &cli.oname, &cli.ocode);
    let oppon_matches2 = match_player(&p2, &cli.ochar, &cli.oname, &cli.ocode);

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
        (true, true, true, true) => Both(p1.port, p2.port),
        (true, _, _, true) => OneWay(p1.port, p2.port),
        (_, true, true, _) => OneWay(p2.port, p1.port),
        _ => NoMatch(),
    }
}

fn match_player(
    player: &game::Player,
    //ignorecase: bool,
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
            Some(n) => n == &np.name,
        }) && (match code {
            None => true,
            Some(c) => c == &np.code,
        })
    })
}

fn match_character(character: &String, id: &External) -> bool {
    let formatted = character.replace(" ", "_").to_ascii_uppercase();
    let formatstr = formatted.as_str();
    if let Ok(charid) = External::try_from(formatstr) {
        charid == *id
    } else {
        false
    }
}
