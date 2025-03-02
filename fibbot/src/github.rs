use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct GitHubComment {
    body: String,
}

pub fn post_comment(
    repo: &str,
    pr_number: u64,
    token: &str,
    comment: &str,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );
    let comment = GitHubComment {
        body: comment.to_string(),
    };

    client
        .post(&url)
        .header("Authorization", format!("token {}", token))
        .json(&comment)
        .send()?;

    Ok(())
}
