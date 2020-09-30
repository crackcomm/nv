use structopt::StructOpt;

pub static DEFAULT_DIFF: u64 = 1000;
pub static DEFAULT_ROUND: u64 = 100;

#[derive(StructOpt, Debug)]
#[structopt(name = "nv")]
pub struct Opt {
    /// NV directory repo path.
    #[structopt(
        short = "u",
        long = "repo",
        default_value = "file://$HOME/.local/nv/$NAMESPACE"
    )]
    pub repo_uri: String,

    /// Ask for URI in secret mode.
    #[structopt(short, long)]
    pub suri: bool,

    /// NV namespace.
    #[structopt(short, long, default_value = "default")]
    pub namespace: String,

    /// Creates repository.
    #[structopt(short, long)]
    pub create: bool,

    /// Force open.
    #[structopt(short, long)]
    pub force: bool,

    /// Debug features.
    #[structopt(long)]
    pub debug: bool,

    /// Round.
    #[structopt(short, long, default_value = "100")]
    pub round: u64,

    /// Difficulty.
    #[structopt(short, long, default_value = "1000")]
    pub diff: u64,

    /// Seed bytes.
    #[structopt(short = "b", long, default_value = "4")]
    pub seed_bytes: usize,
}

pub mod app;
pub mod common;
pub mod errors;
pub mod mnemonic;
pub mod password;
pub mod repo;
pub mod salt;
pub mod seed;
