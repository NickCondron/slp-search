use clap::{Args, Parser, Subcommand};
use std::path;

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    subcommand_required = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn replays(&self) -> &Vec<path::PathBuf> {
        &self.shared_args().replays
    }
    pub fn shared_args(&self) -> &SharedArgs {
        match self.command {
            Commands::Filter(ref filter_args) => &filter_args.shared_args,
            Commands::Search(ref search_args) => &search_args.shared_args,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Quickly filters replays based on metadata
    Filter(Filter),
    ///Search replay for something that happened
    Search(Search),
}

#[derive(Debug, Args)]
pub struct Filter {
    #[clap(flatten)]
    pub shared_args: SharedArgs,

    /// Stage
    #[clap(short, long)]
    pub stage: Option<String>,
}

#[derive(Debug, Args)]
pub struct Search {
    #[clap(flatten)]
    pub shared_args: SharedArgs,

    pub search_string: String,
}

#[derive(Debug, Args)]
pub struct SharedArgs {
    /// Player character
    #[clap(long)]
    pub pchar: Option<String>,

    /// Player netplay name
    #[clap(long)]
    pub pname: Option<String>,

    /// Player netplay connect code (eg. MANG#0)
    #[clap(long)]
    pub pcode: Option<String>,

    /// Opponent character
    #[clap(long)]
    pub ochar: Option<String>,

    /// Opponent netplay name
    #[clap(long)]
    pub oname: Option<String>,

    /// Opponent netplay connect code (eg. MANG#0)
    #[clap(long)]
    pub ocode: Option<String>,

    /// Case insensitive
    #[clap(short, long, global = true)]
    pub ignorecase: bool,

    /// Replays to search
    pub replays: Vec<path::PathBuf>,
}
