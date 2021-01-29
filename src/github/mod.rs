mod github_format;
use github_format::*;
use serde_json::Result;

#[derive(Debug)]
pub struct GithubContext {
    pub event: Event,
    pub repository: String
}

impl GithubContext {
    pub fn from_str(str: &str) -> Result<Self> {
        let github: Result<GithubContextStruct> = serde_json::from_str(str);
        github.map(|github| github.into())
    }

    pub fn labels(&self) -> Vec<Label> {
        let pr_number = match &self.event {
            Event::PullRequest(pull_request) => pull_request.number,
            Event::Unknown => 0
        };
        let pull_request = PullRequest::get(&self.repository, pr_number).expect("Couldn't get PullRequest");
        pull_request.labels
    }
}

impl From<GithubContextStruct> for GithubContext {
    fn from(from: GithubContextStruct) -> Self {
        let event = if let Some(pull_request) = from.event.pull_request {
            Event::PullRequest(pull_request)
        } else {
            Event::Unknown
        };
        let repository = from.repository;
        GithubContext { event, repository }
    }
}

#[derive(Debug)]
pub enum Event {
    PullRequest(PullRequest),
    Unknown
}
