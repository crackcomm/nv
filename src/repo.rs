use dialoguer::theme::ColorfulTheme;
use zbox::RepoOpener;

use crate::{mnemonic, password, seed, Opt, DEFAULT_DIFF, DEFAULT_ROUND};

pub fn open(opt: &Opt) -> zbox::Result<zbox::Repo> {
    let mut mined = 0;
    loop {
        let password = password::prompt(opt.create);
        if opt.debug {
            println!("Your password is {} characters long", password.len());
        }

        let password = if opt.create {
            loop {
                let start = std::time::Instant::now();
                let (mnemonic, password) =
                    seed::mine(&password, opt.seed_bytes, opt.diff, opt.round);
                if opt.debug {
                    println!("Seed mined in {:?}", start.elapsed());
                }
                if mined == 1 {
                    println!("Longer mined seed is not more secure.");
                }
                println!("Mnemonic: {}", mnemonic);

                if dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Keep mnemonic?")
                    .interact()
                    .unwrap()
                {
                    println!("Save this mnemonic. It is impossible to brute-force the password without this mnemonic!");
                    // NOTE: I will implement brute-force for this mnemonic because I will forget it not once and not twice.
                    println!("In contrary it is possible to brute-force this mnemonic by design, if you have the password.\n");
                    if opt.round > DEFAULT_ROUND {
                        println!("In order to open your password repository you will need to use -r {} flag.", opt.round);
                    }
                    if opt.diff > DEFAULT_DIFF {
                        println!("In order to open your password repository you will need to use --diff {} flag.", opt.diff);
                    }
                    break password;
                } else {
                    mined += 1;
                }
            }
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
            .force(opt.force)
            .create(opt.create)
            .cipher(zbox::Cipher::Xchacha)
            .compress(true)
            .open(&opt.repo_uri, &password);
        if repo.is_err() && !opt.create {
            println!("Error: {}", repo.unwrap_err());
            continue;
        }

        return repo;
    }
}
