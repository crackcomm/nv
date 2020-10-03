use dialoguer::{theme::ColorfulTheme, Password};

pub fn prompt(confirm: bool) -> std::io::Result<String> {
    if confirm {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .with_confirmation("Repeat password", "Error: the passwords don't match.")
            .interact()
    } else {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .interact()
    }
}
