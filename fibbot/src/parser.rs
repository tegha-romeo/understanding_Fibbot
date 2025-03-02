pub fn extract_numbers(text: &str) -> Vec<u64> {
    text.split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let text = "This is a PR with numbers 1 2 3 5 8 13";
        let numbers = extract_numbers(text);
        assert_eq!(numbers, vec![1, 2, 3, 5, 8, 13]);
    }
}
