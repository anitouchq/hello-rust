use hello_rust::greeting;

#[test]
fn application_greeting_is_available() {
    assert_eq!(greeting(), "Hello from Rust in Docker! 🦀🐳");
}
