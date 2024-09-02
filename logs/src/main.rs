use std::fs;

fn string_test(a: String, b: &String, c: &str) {}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            println!("{}", text_that_was_read)
        }
        Err(why_this_failed) => {
            println!("{}", why_this_failed)
        }
    }

    string_test(String::from("red"), &String::from("red"), "red")
}
