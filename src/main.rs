fn main() {
    let github_json = std::env::var("GITHUB_JSON").expect("Couldn't get GITHUB_JSON");
    println!("{}", github_json);
}
