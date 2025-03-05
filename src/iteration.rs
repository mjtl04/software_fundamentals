use std::io;

const INPUTS: u32 = 10;

pub fn task1() {
    println!("Enter 10 numbers");

    let mut input = String::new();
    let mut sum_positive: u32 = 0;
    let mut sum_negative: i32 = 0;

    for _ in 0..INPUTS {
        io::stdin()
            .read_line(&mut input)
            .expect("error on read line");

        let mut value: i32 = input.trim().parse().expect("error on parse");
        input.clear();

        if value > 0 {
            sum_positive += value as u32;
        } else {
            sum_negative += value;
        }
    }

    println!("Positive Sum: {sum_positive}");
    println!("Negative Sum: {sum_negative}");
}
