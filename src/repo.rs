use zbox::RepoOpener;

use crate::{errors::Result, mnemonic, password, seed, Opt};

pub fn open(opt: &Opt) -> Result<(zbox::Repo, String)> {
    loop {
        let password = password::prompt(opt.create)?;
        if opt.debug {
            println!("Your password is {} characters long", password.len());
        }

        let password = if opt.create {
            seed::create(opt, &password)?
        } else {
            let mnemonic = mnemonic::prompt()?;
            let start = std::time::Instant::now();
            match seed::mnemonic(&password, mnemonic.as_slice(), opt.diff) {
                Some(password) => {
                    if opt.debug {
                        println!("Hash computed in {:?}", start.elapsed());
                    }
                    password
                }
                None => {
                    println!("Could not compute hash for password.");
                    continue;
                }
            }
        };

        let repo = open_repo(opt, &password, false);
        match repo {
            Ok(repo) => {
                return Ok((repo, password));
            }
            Err(err) => match err {
                zbox::Error::RepoOpened => {
                    println!("Repository is already opened.");
                    println!("Opening in read only mode.");
                    return Ok((open_repo(&opt, &password, true)?, password.to_string()));
                }
                err => {
                    println!("Error: {}.", err);
                    if opt.create {
                        return Err(err.into());
                    }
                }
            },
        }
    }
}

fn open_repo(opt: &Opt, password: &str, force_read_only: bool) -> zbox::Result<zbox::Repo> {
    RepoOpener::new()
        .force(opt.force || force_read_only)
        .ops_limit(zbox::OpsLimit::Sensitive)
        .mem_limit(zbox::MemLimit::Sensitive)
        .create(opt.create)
        .cipher(zbox::Cipher::Xchacha)
        .compress(true)
        .read_only(opt.read_only || force_read_only)
        .open(&opt.repo_uri, &password)
}
