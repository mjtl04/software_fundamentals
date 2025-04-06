use std::{fmt::format, io};

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
    println!("Factoral of {user_num} is: {result}")
}

fn input() -> u32 {
    println!("Enter number to Factorise: ");
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

pub fn task3_main() {
    let height_in_metres = input_height();
    let weight_in_kilos = input_weight();
    calculate_bmi(height_in_metres, weight_in_kilos);
}

fn input_height() -> f32 {
    println!("Height in metres: ");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Enter a valid height in metres");
            }
        }
    }
}

fn input_weight() -> f32 {
    println!("Weight in kilos: ");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Enter a valid weight in kilograms");
            }
        }
    }
}

fn calculate_bmi(height: f32, weight: f32) {
    let bmi: f32 = weight / (height * height);

    let range = if bmi < 18.5 {
        "Underweight"
    } else if (18.5..=24.9).contains(&bmi) {
        "Healthy Weight"
    } else if (25.0..=29.9).contains(&bmi) {
        "Overweight"
    } else {
        "Obese"
    };

    println!("{range} BMI: {bmi:.0}");
}
