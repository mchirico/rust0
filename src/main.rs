use std::fs;

mod folder;
mod mytest;

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Hello, world! Length: {}", len);

    let s = String::from("hello world");
    println!("{}", calculate_length(&s));
    println!("{}", first_word(&s));

    folder::function();


    println!("{}", first_word(&s));

    fs::write("data.dat", s)





}

