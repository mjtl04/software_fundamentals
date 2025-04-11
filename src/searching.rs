use std::{io, usize};

pub fn main() {
    let first_names = [
        "Tim", "Brendan", "Bill", "Hedy", "Barbara", "Elon", "Larry", "Carl", "Guido", "Mark",
    ];
    let last_names = [
        "Berners-Lee",
        "Eich",
        "Gates",
        "Lamarr",
        "Liskov",
        "Musk",
        "Page",
        "Sassenrath",
        "Van-Rassum",
        "Zuckerburg",
    ];

    let user_search = get_input();
    if let Some(index) = linear_search(&user_search, &last_names) {
        println!("{} found at index {index}", first_names[index]);
    } else {
        println!("{user_search} not found");
    }
}

fn get_input() -> String {
    let mut input = String::new();

    println!("Enter a surname: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read last name");
    input.trim().to_lowercase()
}

fn linear_search(input: &str, last_names: &[&str]) -> Option<usize> {
    //linear search
    for index in 0..last_names.iter().len() {
        if last_names[index].to_lowercase() == input {
            return Some(index);
        }
    }
    None
}
