fn foo_is(words: HashMap<String, String>) -> Option<String> {
    words.get("foo").cloned()
}
