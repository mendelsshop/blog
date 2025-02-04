use std::collections::HashMap;

fn main() {
    let words = HashMap::from([("foo".to_owned(), "bar".to_owned())]);
    let otherwords = HashMap::from([("bar".to_owned(), "baz".to_owned())]);
    let mut counter = Counter(0);
    if counter.foo_is(words, |new_counter_reference, x| {
        x.is_some_and(|x| {
            new_counter_reference.bar_is(otherwords, |_, y| y.is_some_and(|y| y != x))
        })
    }) {
        println!("foo is not bar");
    }
}

struct Counter(usize);

impl Counter {
    // very contrived example
    fn foo_is<T>(
        &mut self,
        words: HashMap<String, String>,
        k: impl FnOnce(&mut Self, Option<&'_ String>) -> T,
    ) -> T {
        self.0 += 1;
        k(self, words.get("foo"))
    }
    fn bar_is<T>(
        &mut self,
        words: HashMap<String, String>,
        k: impl FnOnce(&mut Self, Option<&'_ String>) -> T,
    ) -> T {
        self.0 += 1;
        k(self, words.get("bar"))
    }
}
