use zbox::RepoOpener;

use crate::{mnemonic, password, seed, Opt};

pub fn open(opt: &Opt) -> zbox::Result<(zbox::Repo, String)> {
    loop {
        let password = password::prompt(opt.create);
        if opt.debug {
            println!("Your password is {} characters long", password.len());
        }

        let password = if opt.create {
            seed::create(opt, &password)
        } else {
            let mnemonic = mnemonic::prompt();
            let start = std::time::Instant::now();
            let password = seed::mnemonic(&password, mnemonic.as_slice(), opt.diff);
            if opt.debug {
                println!("Hash computed in {:?}", start.elapsed());
            }
            password
        };

        let repo = RepoOpener::new()
            .force(true)
            .ops_limit(zbox::OpsLimit::Sensitive)
            .mem_limit(zbox::MemLimit::Sensitive)
            .create(opt.create)
            .cipher(zbox::Cipher::Xchacha)
            .compress(true)
            .open(&opt.repo_uri, &password);
        if repo.is_err() && !opt.create {
            println!("Error: {}", repo.unwrap_err());
            continue;
        }

        return Ok((repo?, password));
    }
}
