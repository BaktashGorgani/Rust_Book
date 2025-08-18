use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

//fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//    if x.len() > y.len() { x } else { y }
//}

//fn main() {
//    let string1 = String::from("abcd");
//    let string2 = "xyz";
//
//    let result = longest(string1.as_str(), string2);
//    println!("The longest string is {result}");
//}

fn main() {
    let string1 = String::from("abcd");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "hello");
        println!("The longest string is {result}");
    }
}
