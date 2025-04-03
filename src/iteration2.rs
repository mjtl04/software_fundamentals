use rand::Rng;
use std::io;

pub fn task1(){
    println!("matching die");

    let mut die1: u32 = 0;
    let mut die2: u32 = 1;

    let mut rng = rand::thread_rng();
    let mut counter = 0;


    while die1 != die2 {;
        println!("Rolling die..");

        die1 = rng.gen_range(1..=6);
        die2 = rng.gen_range(1..=6);
        counter += 1;
    }
    
    println!("It took {counter} to roll {die1} on both die");

}

pub fn task2(){

    let mut input = String::new();
    println!("Enter number from 0 - 100");

    io::stdin().read_line(&mut input).expect("Error on input");
    let mut value:i32 = input.trim().parse().expect("Error on parse");

    while value < 0 || value > 100 {
        input.clear();
        println!("Enter a valid number from 0 - 100");
        io::stdin().read_line(&mut input).expect("Error on input");
        value = input.trim().parse().expect("Error on parse");
    }

    println!("You entered: {value}");
}

pub fn task3(){

    let mut input = String::new();

    println!("Current Balance: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let mut current_balance:f32 = input.trim().parse().expect("Error on parse");
    input.clear();

    println!("Required Balance: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let required_balance:f32 = input.trim().parse().expect("Error on parse");
    input.clear();

    println!("Interest Rate: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let mut interest:f32 = input.trim().parse().expect("Error on parse");
    input.clear();
    let interest_rate:f32 = interest / 100.0;

    let mut year_count = 0;

    while current_balance < required_balance{
        interest = current_balance * interest_rate;
        current_balance = current_balance + interest;

        year_count += 1;
        println!("Balance after {year_count} years: £{current_balance:.2}");
    }

    println!("It will take {year_count} to achieve £{required_balance:.2}");

}

Survey looks good, thanks for creating it. Couple of small bits I noticed that might be worth taking a look if other agree:

1. "Number of months with current company" - just makes it a bit more clear for the user
2. years in industry - any way of adding some more detail as to what the question is after? maybe "years in the tech/IT industry"
3. "Device used for work" - instead of "which do you use.."
3. Computer manufacturer question to be a drop down. Avoids typos, mismatches [DELL, Acer, HP, Lenovo, Apple, Asus, Chromebook, Apple]
4. Operating System question also as a drop down. [Windows, MacOS, Linux]
5. if you enable the back button does it let the user go back from the review page? if not then maybe best to not have a review page and just show the thank-you page after they finish the questions?

All just small bits to tweak if the others agree, thanks for making it.
