#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishamel. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap_or("No periods means no sentences!");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{i:?}");

    let s: &'static str = "I have a static lifetime.";
}
