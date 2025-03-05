use std::io;

pub fn task1() {
    println!("Post");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error on read line");

    let value: u32 = input.trim().parse().expect("Error on parse");

    match value {
        1 => {
            println!("Bills must be paid")
        }
        2 => {
            println!("Circulars are thrown away")
        }
        3 => {
            println!("Postcards are put on the wall")
        }
        4 => {
            println!("Personal letters are read and have replies written in them")
        }
        _ => {
            println!("Enter a valid number")
        }
    }
}
