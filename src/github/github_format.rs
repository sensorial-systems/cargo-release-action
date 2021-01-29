use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubContextStruct {
    pub event: EventStruct
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStruct {
    pub pull_request: Option<PullRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub labels: Vec<Label>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String
}
