use serde::{Serialize, Deserialize};
use crate::utils::get;

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubContextStruct {
    pub event: EventStruct,
    pub repository: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStruct {
    pub pull_request: Option<PullRequest>,
    #[serde(flatten)]
    pub push: Option<Push>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Push {
    pub commits: Vec<Commit>
}

impl Push {
    pub fn get_pull_request_number(&self, repository: &str) -> Option<u64> {
        let pull_requests: Vec<PullRequest> = get(&format!("https://api.github.com/repos/{}/pulls?state=closed", repository)).expect("Couldn't get Pull Requests.");
        pull_requests.iter().filter(|pull_request| {
            pull_request.merge_commit_sha.is_some()
        }).map(|pull_request| {
            (pull_request.number, pull_request.merge_commit_sha.as_ref().unwrap())
        }).find(|(_pr_number, merge_commit_sha)| {
            self.commits.iter().find(|commit| commit.id == **merge_commit_sha).is_some()
        }).map(|(pr_number, _)| {
            pr_number
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub(crate) labels: Vec<Label>,
    pub merge_commit_sha: Option<String>
}

impl PullRequest {
    pub fn get(repository: &str, number: u64) -> serde_json::Result<Self> {
        get(&format!("https://api.github.com/repos/{}/pulls/{}", repository, number))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String
}
