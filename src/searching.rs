use std::io;

fn get_input() -> String {
    let mut input = String::new();

    println!("Enter a surname: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read last name");
    input.trim().to_lowercase()
}

pub fn task1_main() {
    let user_search = get_input();
    if let Some(index) = linear_search(&user_search) {
        println!("{} found at index {index}", initial_names(index));
    } else {
        println!("{user_search} not found");
    }
}

fn initial_names(index: usize) -> String {
    let first_names = [
        "Tim", "Brendan", "Bill", "Hedy", "Barbara", "Elon", "Larry", "Carl", "Guido", "Mark",
    ];

    first_names[index].to_string()
}

fn linear_search(key: &str) -> Option<usize> {
    //linear search

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
    for index in 0..last_names.iter().len() {
        if last_names[index].to_lowercase() == key {
            return Some(index);
        }
    }
    None
}

pub fn task2_main() {
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

    let left: usize = 0;
    let right: usize = last_names.len() - 1;

    if let Some(index) = ternary_search(&user_search, &last_names, left, right) {
        println!("{} found at index {index}", first_names[index]);
    } else {
        println!("{user_search} not found");
    }
}

fn ternary_search(key: &str, last_names: &[&str], left: usize, right: usize) -> Option<usize> {
    if left > right {
        None
    } else {
        let mid_first = left + (right - left) / 3;
        let mid_second = mid_first + (right - left) / 3;

        if last_names[mid_first].to_lowercase() == key {
            return Some(mid_first);
        }

        if last_names[mid_second].to_lowercase() == key {
            return Some(mid_second);
        }

        if key < &last_names[mid_first].to_lowercase() {
            return ternary_search(key, last_names, left, mid_first - 1);
        }

        if key > &last_names[mid_second].to_lowercase() {
            return ternary_search(key, last_names, mid_second + 1, right);
        }

        ternary_search(key, last_names, mid_first + 1, mid_second - 1)
    }
}
