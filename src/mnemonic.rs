use dialoguer::{theme::ColorfulTheme, Password};

fn prompt_() -> String {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Mnemonic")
        .interact()
        .unwrap()
        .replace(" ", "-")
}

pub fn prompt() -> Vec<u8> {
    loop {
        let input = prompt_();
        let mut decoded = Vec::<u8>::new();
        match mnemonic::decode(input.as_bytes(), &mut decoded) {
            Ok(_) => return decoded,
            Err(_) => continue,
        }
    }
}
