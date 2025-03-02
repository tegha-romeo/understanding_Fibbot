use std::env;

pub fn parse_input(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
