use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    println!(
        "cargo:rustc-env=GIT_HASH={}",
        String::from_utf8(output.stdout).unwrap()
    );
}
