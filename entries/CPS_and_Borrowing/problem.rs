use std::collections::HashMap;

fn main() {
    let words = HashMap::from([("foo".to_owned(), "bar".to_owned())]);
    if foo_is(words).is_some_and(|x| x == "bar") {
        println!("foo is bar");
    }
}

// very contrived example
fn foo_is<'a>(words: HashMap<String, String>) -> Option<&'a String> {
    words.get("foo")
}
