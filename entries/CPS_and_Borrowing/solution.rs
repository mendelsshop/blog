fn foo_is<T>(words: HashMap<String, String>, k: impl FnOnce(Option<&'_ String>) -> T) -> T {
    k(words.get("foo"))
}
