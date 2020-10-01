use dialoguer::theme::ColorfulTheme;
use hex::ToHex;

use crate::{
    common::{argon2d_simple, rand_bytes},
    salt, Opt, DEFAULT_DIFF, DEFAULT_ROUND,
};

pub fn mine(password: &str, sbytes: usize, diff: u64, round: u64) -> (String, String) {
    loop {
        let bytes = rand_bytes(sbytes);
        let mnemonic = mnemonic::to_string(&bytes);

        match salt::mine(&password, &bytes, diff, round) {
            Some(salt) => {
                let salt = argon2d_simple(&password, &salt).encode_hex::<String>();
                return (mnemonic, salt);
            }
            None => continue,
        }
    }
}

pub fn mnemonic(password: &str, bytes: &[u8], diff: u64) -> String {
    let salt = salt::mine(&password, bytes, diff, u64::MAX).unwrap();
    argon2d_simple(&password, &salt).encode_hex::<String>()
}

pub fn create(opt: &Opt, password: &str) -> String {
    let mut mined = 0;
    loop {
        let start = std::time::Instant::now();
        let (mnemonic, password) = mine(password, opt.seed_bytes, opt.diff, opt.round);
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
                println!(
                    "In order to open your password repository you will need to use -r {} flag.",
                    opt.round
                );
            }
            if opt.diff > DEFAULT_DIFF {
                println!("In order to open your password repository you will need to use --diff {} flag.", opt.diff);
            }
            break password;
        } else {
            mined += 1;
        }
    }
}
