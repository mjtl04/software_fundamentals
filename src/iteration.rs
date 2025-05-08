use std::io;
const INPUT_ITERATIONS: u32 = 10;

pub fn task1() {
    println!("Enter 10 numbers");

    let mut input = String::new();
    let mut sum_positive: u32 = 0;
    let mut sum_negative: i32 = 0;

    for _ in 0..INPUT_ITERATIONS {
        io::stdin()
            .read_line(&mut input)
            .expect("error on read line");

        let value: i32 = input.trim().parse().expect("error on parse");
        input.clear();

        if value >= 0 {
            sum_positive += value as u32;
        } else {
            sum_negative += value;
        }
    }

    println!("Positive Sum: {sum_positive}");
    println!("Negative Sum: {sum_negative}");
}

pub fn task2() {
    println!("Enter 10 numbers");

    let mut input = String::new();
    let mut sum_positive: u32 = 0;
    let mut sum_negative: i32 = 0;

    let mut positive_count: u32 = 0;
    let mut negative_count: u32 = 0;

    for _ in 0..INPUT_ITERATIONS {
        io::stdin()
            .read_line(&mut input)
            .expect("error on read line");

        let value: i32 = input.trim().parse().expect("error on parse");
        input.clear();

        if value >= 0 {
            sum_positive += value as u32;
            positive_count += 1;
        } else {
            sum_negative += value;
            negative_count += 1;
        }
    }

    let postive_avg: f32 = sum_positive as f32 / positive_count as f32;
    let negative_avg: f32 = sum_negative as f32 / negative_count as f32;

    println!("Positive Sum: {sum_positive}");
    println!("Positive Avg: {postive_avg:.2}");

    println!("Negative Sum: {sum_negative}");
    println!("Negative Avg: {negative_avg:.2}");
}
