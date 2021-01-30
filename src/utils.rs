use std::io::Read;
use serde::de::DeserializeOwned;
use std::process::Command;

pub fn get<T: DeserializeOwned>(url: &str) -> serde_json::Result<T> {
    let client = reqwest::blocking::Client::builder().user_agent("cargo-release-action").build().expect("Couldn't create client.");
    let mut res = client.execute(client.get(url).build().expect("Couldn't create request.")).expect("Couldn't get response");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Couldn't read body to string.");
    serde_json::from_str(&body)
}

fn execute(command: &str, args: &[&str]) -> Result<(), String> {
    println!("Executing: {} {:?}", command, args);
    let status = Command::new(command)
        .args(args.iter())
        .status()
        .expect("Couldn't get Output.");
    if status.success() {
        Ok(())
    } else {
        Err(format!("{} {:?}: execution failed", command, args))
    }
}

pub fn publish(release: &str, cargo_token: &str) -> Result<(), String> {
    execute("git", &["config", "--local", "user.email", "41898282+github-actions[bot]@users.noreply.github.com"])?;
    execute("git", &["config", "--local", "user.name", "github-actions[bot]"])?;
    execute("cargo", &["login", &cargo_token])?;
    execute("cargo", &["install", "cargo-release"])?;
    execute("cargo", &["release", release, "--no-confirm"])
}

pub fn check_publish() -> Result<(), String> {
    execute("cargo", &["publish", "--dry-run"])
}