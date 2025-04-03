use std::io;

pub fn task1() {
    println!("Post");
    println!("Enter the type of letter you have received: e.g 3: ");
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

pub fn task2() {
    println!("What day is it?");
    println!("Enter the day of the week: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error on read line");

    let day: u32 = input.trim().parse().expect("Error on parse");

    match day {
        1..=5 => {
            println!("It is a week day")
        }
        6 => {
            println!("It's Saturday")
        }
        7 => {
            println!("It's Sunday")
        }
        _ => println!("Enter a valid week day"),
    }
}

pub fn task3() {
    println!("Grade Classifier");
    println!("Enter your grade");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error on read line");

    let grade: u32 = input.trim().parse().expect("Error on parse");

    match grade {
        0..=39 => {
            println!("Referral")
        }
        40..=49 => {
            println!("Pass")
        }
        50..=69 => {
            println!("Merit")
        }
        70..=100 => {
            println!("Distinction")
        }
        _ => {
            println!("Invalid grade")
        }
    }
}
