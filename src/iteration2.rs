use rand::Rng;
use std::io;

const RANGE: std::ops::Range<u32> = 1..6;

pub fn task1() {
    println!("matching die");

    let mut die1: u32 = 0;
    let mut die2: u32 = 1;

    let mut rng = rand::thread_rng();
    let mut counter = 0;

    while die1 != die2 {
        die1 = rng.gen_range(RANGE);
        die2 = rng.gen_range(RANGE);

        println!("die 1 is {die1}, die 2 is {die2}");
        counter += 1;
    }

    println!("It took {counter} goes to throw {die1} on both dice");
}

pub fn task2() {
    let mut input = String::new();
    println!("Enter number from 0 - 100");

    io::stdin().read_line(&mut input).expect("Error on input");
    let mut value: i32 = input.trim().parse().expect("Error on parse");

    while !(0..=100).contains(&value) {
        input.clear();
        println!("Enter a valid number from 0 - 100");
        io::stdin().read_line(&mut input).expect("Error on input");
        value = input.trim().parse().expect("Error on parse");
    }

    println!("You entered: {value}");
}

pub fn task3() {
    let mut input = String::new();

    println!("Current Balance: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let mut current_balance: f32 = input.trim().parse().expect("Error on parse");
    input.clear();

    println!("Required Balance: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let required_balance: f32 = input.trim().parse().expect("Error on parse");
    input.clear();

    println!("Interest Rate: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let mut interest: f32 = input.trim().parse().expect("Error on parse");
    input.clear();
    let interest_rate: f32 = interest / 100.0;

    let mut year_count = 0;

    while current_balance < required_balance {
        interest = current_balance * interest_rate;
        current_balance += interest;

        year_count += 1;
        println!("Balance after {year_count} years: £{current_balance:.2}");
    }

    println!("It will take {year_count} to achieve £{required_balance:.2}");
}
