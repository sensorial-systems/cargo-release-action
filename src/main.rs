//! TODO: PR information from GITHUB_JSON is possibly outdated. If we want to get an updated one
//!  we need to get the current PR status from the API. We are already doing it in GithubContext.labels(),
//!  but we want our API to reflect this.

use cargo_release_action::*;
use cargo_release_action::utils::*;

fn main() {
    std::env::set_var("GITHUB_JSON", include_str!("../tests/push.json"));
    std::env::set_var("PATCH_LABEL", "patch");
    std::env::set_var("MINOR_LABEL", "minor");
    std::env::set_var("MAJOR_LABEL", "major");
    let github_json = std::env::var("GITHUB_JSON").expect("Couldn't get GITHUB_JSON");
    println!("{}", github_json);
    let github = GithubContext::from_str(&github_json).expect("Couldn't parse JSON.");
    println!("Github: {:#?}", github);
    let release: Option<Release> = (&github).into();
    match &github.event {
        Event::PullRequest(_) => {
            println!("The semver {:?} number will be bumped on merge.", release.expect("Release label not present."));
            check_publish();
        },
        Event::Push(_) => {
            // If release.is_none(), then the Event::Push probably didn't come from a pull request.
            if let Some(release) = release {
                publish(&format!("{:?}", release).to_lowercase(), "");
            }
        },
        Event::Unknown => ()
    }
}
