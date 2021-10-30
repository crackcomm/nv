use structopt::StructOpt;

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

    /// NV namespace.
    #[structopt(short, long, default_value = "default")]
    pub namespace: String,

    /// Creates repository.
    #[structopt(short, long)]
    pub create: bool,

    /// Force open.
    #[structopt(short, long)]
    pub force: bool,

    /// Read only.
    #[structopt(long = "read", help = "Open with read only access.")]
    pub read_only: bool,

    /// Debug features.
    #[structopt(long)]
    pub debug: bool,

    /// Round.
    #[structopt(short, long, default_value = "256")]
    pub round: u64,

    /// Difficulty.
    #[structopt(short, long, default_value = "1")]
    pub diff: u64,

    /// Seed bytes.
    #[structopt(short = "b", long, default_value = "2")]
    pub seed_bytes: usize,
}

pub static DEFAULT_DIFF: u64 = 1;
pub static DEFAULT_ROUND: u64 = 256;

pub mod app;
pub mod common;
pub mod errors;
pub mod mnemonic;
pub mod password;
pub mod repo;
pub mod salt;
pub mod seed;
