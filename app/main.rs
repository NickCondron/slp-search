pub mod args;
pub mod regex;

use clap::Parser;
use peppi::model::enums::{character::External, stage::Stage};
use peppi::serde::de;
use std::{fs, io};

use slp_filter as filter;
use slp_filter::Filter;

use crate::regex::*;
use args::{Cli, Commands, SharedArgs};

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
        let opts = Some(de::Opts {
            skip_frames: true,
            debug_dir: None,
        });
        let game = peppi::game(&mut buf, opts.as_ref(), None);
        if let Err(e) = game {
            eprintln!("{}", e);
            continue;
        }
        let game = game.unwrap();

        let args: &SharedArgs = cli.shared_args();
        let player = filter::Player {
            character: args.pchar.as_ref().and_then(|c| External::try_match(c)),
            name: args.pname.clone(),
            code: args.pcode.clone(),
        };
        let opponent = filter::Player {
            character: args.ochar.as_ref().and_then(|c| External::try_match(c)),
            name: args.oname.clone(),
            code: args.ocode.clone(),
        };
        let mut filter = filter::Game::new();
        filter.add_player(player);
        filter.add_player(opponent);

        match cli.command {
            Commands::Filter(ref filter_args) => {
                filter_args
                    .stage
                    .as_ref()
                    .and_then(|s| Stage::try_match(s))
                    .map(|s| filter.set_stage(s));
                if filter.is_match(&game) {
                    println!("{:#?}", path.to_string_lossy());
                }
            }
            Commands::Search(ref _search_args) => {
                todo!();
            }
        }
    }
}
