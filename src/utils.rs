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

fn execute(command: &str, args: &[&str]) -> Result<String, String> {
    println!("Executing: {} {:?}", command, args);
    let output = Command::new(command)
        .args(args.iter())
        .output()
        .expect("Couldn't get Output.");
    let stdout = String::from_utf8(output.stdout)
        .map(|stdout| {
            println!("{}", stdout);
            stdout
        })
        .map_err(|err| {
            err.to_string()
        });
    if output.status.success() {
        stdout
    } else {
        Err(format!("{} {:?}: execution failed", command, args))
    }
}

pub fn publish(release: &str, github_token: &str, cargo_token: &str) -> Result<String, String> {
    execute("git", &["config", "--local", "user.email", "41898282+github-actions[bot]@users.noreply.github.com"])?;
    execute("git", &["config", "--local", "user.name", "github-actions[bot]"])?;
    execute("cargo", &["login", &cargo_token])?;
    execute("cargo", &["install", "cargo-release"])?;
    execute("cargo", &["release", release, "--no-confirm"])
}

pub fn check_publish() -> Result<(), String> {
    let output = execute("cargo", &["publish", "--dry-run"])?;
    output.find("warning")
        .map(|_| ())
        .ok_or_else(|| "Check publish failed.".to_string())
}