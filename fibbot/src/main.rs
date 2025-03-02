mod fibonacci;
mod github;
mod parser;
mod utils;

use github::post_comment;
use parser::extract_numbers;
use std::env;
use utils::parse_input;

fn main() {
    let enable_fib = parse_input("INPUT_ENABLE_FIB", "true");
    let max_threshold = parse_input("INPUT_MAX_THRESHOLD", "100");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER").expect("PR_NUMBER not set");
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let pr_content = "This is a PR with numbers 1 2 3 5 8 13";
    let numbers = extract_numbers(pr_content);

    if enable_fib == "true" {
        let mut comment_body = String::new();
        for number in numbers {
            if number <= max_threshold.parse::<u64>().unwrap() {
                let fib = fibonacci::fibonacci(number);
                comment_body.push_str(&format!("Fibonacci({}) = {}\n", number, fib));
            }
        }

        if !comment_body.is_empty() {
            post_comment(
                &repo,
                pr_number.parse::<u64>().unwrap(),
                &token,
                &comment_body,
            )
            .unwrap();
        }
    }
}
