fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    let shit;
    if string1.len() > string2.len() {
        shit = string1
    } else {
        shit = string2
    }
    &shit
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("another string");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result)
}
