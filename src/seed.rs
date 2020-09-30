use hex::ToHex;

use crate::{common::rand_bytes, salt};

pub fn mine(password: &str, sbytes: usize, diff: u64, round: u64) -> (String, String) {
    loop {
        let bytes = rand_bytes(sbytes);
        let mnemonic = mnemonic::to_string(&bytes);

        match salt::mine(&password, &bytes, diff, round) {
            Some(salt) => {
                let salt = argon2rs::argon2d_simple(&password, &salt).encode_hex::<String>();
                return (mnemonic, salt);
            }
            None => continue,
        }
    }
}

pub fn mnemonic(password: &str, bytes: &[u8], diff: u64) -> String {
    let salt = salt::mine(&password, bytes, diff, u64::MAX).unwrap();
    argon2rs::argon2d_simple(&password, &salt).encode_hex::<String>()
}
