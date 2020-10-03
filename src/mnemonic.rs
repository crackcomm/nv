use dialoguer::{theme::ColorfulTheme, Password};

fn prompt_() -> std::io::Result<String> {
    Ok(Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Mnemonic")
        .interact()?
        .replace(" ", "-"))
}

pub fn prompt() -> std::io::Result<Vec<u8>> {
    loop {
        let input = prompt_()?;
        let mut decoded = Vec::<u8>::new();
        match mnemonic::decode(input.as_bytes(), &mut decoded) {
            Ok(_) => return Ok(decoded),
            Err(_) => continue,
        }
    }
}
