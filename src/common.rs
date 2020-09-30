use std::fmt::Display;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};
use rand::Rng;

pub fn secret_prompt<T: Display>(prompt: T) -> String {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt.to_string())
        .interact()
        .unwrap()
}

pub fn confirm_prompt<T: Display>(prompt: T) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt.to_string())
        .interact()
        .unwrap()
}

pub fn input_prompt<T: Display>(prompt: T) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .allow_empty(true)
        .with_prompt(&prompt.to_string())
        .interact()
        .unwrap()
}

pub fn rand_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<u8>()).collect::<Vec<_>>()
}

pub fn rand_password(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| loop {
            let c = rng.gen::<u8>();
            if c > 32 && c < 127 {
                break c;
            }
        })
        .collect::<Vec<_>>()
}
