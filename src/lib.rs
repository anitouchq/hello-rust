/// Returns the application's greeting.
pub fn greeting() -> &'static str {
    "Hello from Rust in Docker! 🦀🐳"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_not_empty() {
        assert!(!greeting().is_empty());
    }

    #[test]
    fn greeting_mentions_rust() {
        assert!(greeting().contains("Rust"));
    }
}
