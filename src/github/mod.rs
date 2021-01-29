mod github_format;
use github_format::*;

use serde_json::Result;

#[derive(Debug)]
pub struct GithubContext {
    pub event: Event
}

impl GithubContext {
    pub fn from_str(str: &str) -> Result<Self> {
        let github: Result<GithubContextStruct> = serde_json::from_str(str);
        github.map(|github| github.into())
    }

    pub fn labels(&self) -> Vec<Label> {
        match &self.event {
            Event::PullRequest(pull_request) => pull_request.labels.clone(),
            Event::Unknown => Vec::new()
        }
    }
}

impl From<GithubContextStruct> for GithubContext {
    fn from(from: GithubContextStruct) -> Self {
        let event = if let Some(pull_request) = from.event.pull_request {
            Event::PullRequest(pull_request)
        } else {
            Event::Unknown
        };
        GithubContext { event }
    }
}

#[derive(Debug)]
pub enum Event {
    PullRequest(PullRequest),
    Unknown
}
