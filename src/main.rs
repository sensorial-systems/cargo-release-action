mod github;
use github::*;
use std::process::Command;

#[derive(Debug)]
pub enum Release {
    Major,
    Minor,
    Patch
}

impl From<&GithubContext> for Option<Release> {
    fn from(github: &GithubContext) -> Self {
        let major_label = std::env::var("MAJOR_LABEL").expect("Couldn't get MAJOR_LABEL");
        let minor_label = std::env::var("MINOR_LABEL").expect("Couldn't get MINOR_LABEL");
        let patch_label = std::env::var("PATCH_LABEL").expect("Couldn't get PATCH_LABEL");

        let labels = github.labels();
        if let Some(_) = labels.iter().find(|label| label.name == patch_label) {
            Some(Release::Patch)
        } else if let Some(_) = labels.iter().find(|label| label.name == minor_label) {
            Some(Release::Minor)
        } else if let Some(_) = labels.iter().find(|label| label.name == major_label) {
            Some(Release::Major)
        } else {
            None
        }
    }
}

fn execute(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args.iter())
        .status()
        .expect("Couldn't get ExitStatus.");
    if !status.success() {
        panic!("Command execution failed.");
    }
}

fn main() {
    let github_json = std::env::var("GITHUB_JSON").expect("Couldn't get GITHUB_JSON");
    let github = GithubContext::from_str(&github_json).expect("Couldn't parse JSON.");
    let release: Option<Release> = (&github).into();
    match &github.event {
        Event::PullRequest(_) => {
            println!("The semver {:?} number will be bumped on merge.", release.expect("Release label not present"));
            execute("cargo", &["publish", "--dry-run"]);
        },
        _ => ()
    }
}
