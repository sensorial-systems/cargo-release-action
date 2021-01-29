pub use github::*;

mod github;
pub mod utils;

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
