use peppi::model::{
    enums::{character::External, stage::Stage},
    game,
};

use itertools::Itertools;

pub trait Filter {
    type Candidate;
    fn is_match(&self, c: &Self::Candidate) -> bool;
}

#[derive(Clone, Debug)]
pub struct Game {
    players: Vec<Player>,
    num_players: Option<u8>,
    stage: Option<Stage>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: vec![],
            num_players: None,
            stage: None,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn num_players(&mut self, n: u8) {
        if n < 2 || n > 4 {
            panic!("Invalid number of players {}", n);
        }
        self.num_players = Some(n);
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = Some(stage);
    }
}

impl Filter for Game {
    type Candidate = game::Game;
    fn is_match(&self, game: &Self::Candidate) -> bool {
            self.stage.map(|s| s == game.start.stage).unwrap_or(true)
            && self
                .num_players
                .map(|s| s as usize == game.start.players.len())
                .unwrap_or(true)
            && {
                let filters = self.players.iter();
                let players = game.start.players.iter();

                if filters.len() >= players.len() {
                    filters
                        .permutations(players.len())
                        .find(move |filter_perm| {
                            filter_perm
                                .iter()
                                .zip(players.clone())
                                .all(|(filter, player)| filter.is_match(player))
                        })
                        .is_some()
                } else {
                    players
                        .permutations(filters.len())
                        .find(move |player_perm| {
                            player_perm
                                .iter()
                                .zip(filters.clone())
                                .all(|(player, filter)| filter.is_match(player))
                        })
                        .is_some()
                }
            }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub character: Option<External>,
    pub name: Option<String>,
    pub code: Option<String>,
}

impl Filter for Player {
    type Candidate = game::Player;
    fn is_match(&self, player: &Self::Candidate) -> bool {
        self.character
            .map(|c| c == player.character)
            .unwrap_or(true)
            && match (
                self.name.as_ref(),
                self.code.as_ref(),
                player.netplay.as_ref(),
            ) {
                (None, None, _) => true,
                (_, _, None) => false,
                (name, code, Some(netplay)) => {
                    name.map(|n| *n == netplay.name).unwrap_or(true)
                        && code.map(|c| *c == netplay.code).unwrap_or(true)
                }
            }
    }
}
