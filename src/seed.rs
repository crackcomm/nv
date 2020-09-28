use hex::ToHex;
use rand::Rng;

pub fn mine(password: &str, diff: u64, round: u64) -> (String, String) {
    let mut rng = rand::thread_rng();
    loop {
        let bytes = [rng.gen(), rng.gen()];
        let mnemonic = mnemonic::to_string(&bytes);

        match crate::salt::mine(&password, &bytes, diff, round) {
            Some(salt) => {
                let salt = argon2rs::argon2d_simple(&password, &salt).encode_hex::<String>();
                return (mnemonic, salt);
            }
            None => continue,
        }
    }
}

pub fn mnemonic(password: &str, bytes: &[u8], diff: u64) -> String {
    let salt = crate::salt::mine(&password, bytes, diff, u64::MAX).unwrap();
    argon2rs::argon2d_simple(&password, &salt).encode_hex::<String>()
}
