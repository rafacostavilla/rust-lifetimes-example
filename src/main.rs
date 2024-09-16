fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("Longest string");
    let string2 = "short string";

    let result = longest(string1.as_str(), &string2);
    println!("The longest string is '{result}'");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt{
        part: first_sentence,
    };
    println!("First sentence: '{}'", i.part);
}
