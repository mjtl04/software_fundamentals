use std::io;

pub fn task1() {
    //selection
    let mut input = String::new();

    println!("Input the Cost");
    io::stdin().read_line(&mut input).expect("error on input");
    let cost: f32 = input.trim().parse().expect("error on assigning");

    if cost >= 0.0 && cost <= 1000.0 {
        if cost > 500.0 {
            println!("You get a 20% discount!");
        } else if cost > 250.0 {
            println!("You get a 10% discount!");
        } else {
            println!("No discount");
        }
    } else {
        println!("Input a valid number between 0 and 1000");
    }
}
