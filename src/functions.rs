use core::num;
use std::io;

pub fn task1_main() {
    let numbers = get_input();
    let min: u32 = smaller(&numbers);
    println!("Smallest number is: {min}");
}

fn get_input() -> [u32; 3] {
    println!("Input 3 Numbers: ");
    let mut numbers: [u32; 3] = [0; 3];

    for i in 0..3 {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<u32>() {
                Ok(value) => {
                    numbers[i] = value;
                    break;
                }
                Err(_) => {
                    println!("Enter a valid number");
                }
            };
        }
    }
    numbers
}

fn smaller(numbers: &[u32]) -> u32 {
    *numbers.iter().min().unwrap()
}

pub fn task2_main() {
    let user_num = input();
    let result = factorial(user_num);
    println!("Factoral: {result}")
}

fn input() -> u32 {
    println!("Enter a number for the Factorial");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read message");

        match input.trim().parse::<u32>() {
            Ok(value) => {
                return value;
            }
            Err(_) => {
                println!("Enter a valid number")
            }
        };
    }
}

fn factorial(number: u32) -> u32 {
    let mut fanswer: u32 = 1;

    for i in (2..=number).rev() {
        fanswer *= i;
    }

    fanswer
}
