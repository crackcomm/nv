use dialoguer::{theme::ColorfulTheme, Password};

pub fn prompt(confirm: bool) -> String {
    if confirm {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .with_confirmation("Repeat password", "Error: the passwords don't match.")
            .interact()
            .unwrap()
    } else {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .interact()
            .unwrap()
    }
}
