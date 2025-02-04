use std::collections::HashMap;

fn main() {
    let words = HashMap::from([("foo".to_owned(), "bar".to_owned())]);
    if foo_is(words, |x| x.is_some_and(|x| x == "bar")) {
        println!("foo is bar");
    }
}

// very contrived example
fn foo_is<T>(words: HashMap<String, String>, k: impl FnOnce(Option<&'_ String>) -> T) -> T {
    k(words.get("foo"))
}
