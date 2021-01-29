#[cfg(test)]
mod test {
    use cargo_release_action::GithubContext;

    #[test]
    fn test() {
        let github_json = include_str!("test.json");
        let github = GithubContext::from_str(&github_json).expect("Couldn't parse JSON.");
        println!("Github: {:#?}", github);
        println!("PR labels: {:#?}", github.labels());
    }
}