use std::io::Read;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::process::Command;

pub fn get<T: DeserializeOwned>(url: &str) -> serde_json::Result<T> {
    let client = reqwest::blocking::Client::builder().user_agent("cargo-release-action").build().expect("Couldn't create client.");
    let mut res = client.execute(client.get(url).build().expect("Couldn't create request.")).expect("Couldn't get response");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Couldn't read body to string.");
    serde_json::from_str(&body)
}

pub fn execute(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args.iter())
        .status()
        .expect("Couldn't get ExitStatus.");
    if !status.success() {
        panic!("Command execution failed.");
    }
}

pub fn check_publish() {
    let command = "cargo";
    let args = &["publish", "--dry-run"];
    let output = Command::new(command)
        .args(args.iter())
        .output()
        .expect("Couldn't get Output.");
    if output.status.success() {
        let output = String::from_utf8(output.stdout).expect("Couldn't parse utf8.");
        println!("{}", output);
        if let Some(_) = output.find("warning") {
            std::process::exit(-1);
        }
    } else {
        panic!("Command execution failed.");
    }
}