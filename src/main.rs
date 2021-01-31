//! TODO: PR information from GITHUB_JSON is possibly outdated. If we want to get an updated one
//!  we need to get the current PR status from the API. We are already doing it in GithubContext.labels(),
//!  but we want our API to reflect this.

use cargo_release_action::*;
use cargo_release_action::utils::*;

fn main() {
    let github_json = std::env::var("GITHUB_JSON").expect("Couldn't get GITHUB_JSON");
    let github = GithubContext::from_str(&github_json).expect("Couldn't parse JSON.");
    let release: Option<Release> = (&github).into();
    match &github.event {
        Event::PullRequest(_) => {
            println!("Checking release...");
            println!("The semver {:?} number will be bumped on merge.", release.expect("Release label not present."));
            check_publish().expect("Check publish failed.");
        },
        Event::Push(_) => {
            // If release.is_none(), then the Event::Push probably didn't come from a pull request.
            if let Some(release) = release {
                println!("Releasing...");
                let release = format!("{:?}", release).to_lowercase();
                let cargo_token  = std::env::var("CARGO_TOKEN").expect("Couldn't get CARGO_TOKEN. Remember to set the cargo-token input in your 'on push' action.");
                let github_user_name = std::env::var("GITHUB_USER_NAME").expect("Couldn't get GITHUB_USER_NAME.");
                let github_token = std::env::var("GITHUB_TOKEN").expect("Couldn't get GITHUB_TOKEN.");
                let repository = &github.repository;
                publish(&release, repository, &github_user_name, &github_token, &cargo_token).expect("Publish failed.");
            } else {
                println!("Not releasing.");
            }
        },
        Event::Unknown => ()
    }
}
