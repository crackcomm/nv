use hex::ToHex;
use indicatif::ProgressBar;
use primitive_types::U256;

static ISALT_SEED: &str = "c13b2d1bd9a5920c3697ac7c992c029bbb6240ebddb8f86e44a504f7c7dbfab1";

pub fn mine(password: &str, seed: &[u8], diff: u64, round: u64) -> Option<String> {
    let salt_target: U256 = U256::MAX / U256::from(diff);

    let iseed = seed.encode_hex::<String>();
    let ephseed = argon2rs::argon2d_simple(&iseed, ISALT_SEED).encode_hex::<String>();
    let mut current_hash = argon2rs::argon2d_simple(&password, &ephseed);
    let mut nonce = U256::from(current_hash);

    let bar = ProgressBar::new(round);

    let mut iter = 0;
    let salt = loop {
        let seed = current_hash.encode_hex::<String>();
        let current_nonce =
            argon2rs::argon2d_simple(&format!("{:#?}", nonce), &seed).encode_hex::<String>();

        // Calculate incremented nonce using previous result as seed
        let seed = current_nonce.chars().rev().collect::<String>();
        let hash = argon2rs::argon2d_simple(&current_nonce, &seed);
        let salt = U256::from(hash);
        if salt < salt_target {
            break hash;
        }

        // Increment nonce by one
        iter += 1;
        nonce += U256::one();
        current_hash = hash;
        bar.inc(1);

        if iter == round {
            bar.finish_with_message("Mining on different seed.");
            return None;
        }
    };

    Some(salt.encode_hex::<String>())
}
