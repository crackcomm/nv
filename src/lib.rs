use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "nv")]
pub struct Opt {
    /// NV directory repo path.
    #[structopt(short, long, parse(from_os_str))]
    pub repo_dir: Option<PathBuf>,

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
    #[structopt(short, long)]
    pub debug: bool,

    /// Round.
    #[structopt(long, default_value = "10")]
    pub round: u64,

    /// Difficulty.
    #[structopt(long, default_value = "1000000")]
    pub diff: u64,
}

pub mod mnemonic;
pub mod password;
pub mod salt;
pub mod seed;
