use serde::{Serialize, Deserialize};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubContextStruct {
    pub event: EventStruct,
    pub repository: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStruct {
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub(crate) labels: Vec<Label>
}

impl PullRequest {
    pub fn get(repository: &str, number: u64) -> serde_json::Result<Self> {
        let client = reqwest::blocking::Client::builder().user_agent("cargo-release-action").build().expect("Couldn't create client.");
        let mut res = client.execute(client.get(&format!("https://api.github.com/repos/{}/pulls/{}", repository, number)).build().expect("Couldn't create request.")).expect("Couldn't get response");
        let mut body = String::new();
        res.read_to_string(&mut body).expect("Couldn't read body to string.");
        serde_json::from_str(&body)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String
}
