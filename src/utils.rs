use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use std::process::Command;

pub fn get<T: DeserializeOwned>(url: &str) -> serde_json::Result<T> {
    let client = reqwest::blocking::Client::builder().user_agent("cargo-release-action").build().expect("Couldn't create client.");
    let mut res = client.execute(client.get(url).build().expect("Couldn't create request.")).expect("Couldn't get response");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Couldn't read body to string.");
    serde_json::from_str(&body)
}

fn execute_with_output(command: &str, args: &[&str]) -> Result<String, String> {
    println!("Executing: {} {:?}", command, args);
    let output = Command::new(command)
        .args(args.iter())
        .output()
        .expect("Couldn't get Output.");

    if output.status.success() {
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(format!("{}\n{}", stderr, stdout))
    } else {
        Err(format!("{} {:?}: execution failed", command, args))
    }
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
    execute("cargo", &["release", release, "--no-confirm", "--skip-publish"])?;
    execute("cargo", &["release", "--no-confirm", "--skip-push"])
}

pub fn check_publish() -> Result<(), String> {
    let output = execute_with_output("cargo", &["publish", "--dry-run"])?;
    let warning_count = output
        .lines()
        .filter(|line| {
            line.find("warning").is_some()
        }).count();
    // Because it will always print "warning: aborting upload due to dry run", we count if we have
    // more warnings than 1.
    if warning_count > 1 {
        Err("Can't publish crate.".to_string())
    } else {
        Ok(())
    }
}