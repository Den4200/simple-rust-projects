use std::fmt::Display;
use std::io;


fn main() {
    println!("Enter two strings.");

    let str1 = input("1. ");
    println!();
    let str2 = input("2. ");

    let longest_string = longest(str1.as_str(), str2.as_str());

    println!("\nThe longest string is: \"{}\".", longest_string);

    let announcement = input("\nEnter an announcement.");

    println!("\nThe longest string is: \"{}\".", longest_with_an_announcement(&str1, &str2, &announcement));
}


fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    response.trim().to_string()
}
