use std::io;

pub fn task1() {
    //selection
    let mut input = String::new();

    println!("Input the Cost");
    io::stdin().read_line(&mut input).expect("error on input");
    let cost: f32 = input.trim().parse().expect("error on parsing");

    if (0.0..=1000.0).contains(&cost) {
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

pub fn task2() {
    const ADULT: f32 = 10.50;
    const CHILD: f32 = 7.30;
    const CONC: f32 = 8.40;
    const POSTAGE_COST: f32 = 2.34;

    let mut input = String::new();

    //Input
    println!("Adults: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let no_adults: u32 = input.trim().parse().expect("error on parse");
    input.clear();

    println!("Children: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let no_children: u32 = input.trim().parse().expect("error on parse");
    input.clear();

    println!("Concessions: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let no_concessions: u32 = input.trim().parse().expect("error on parse");
    input.clear();

    println!("Collection in Person? yes/no: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let collection: String = input.trim().to_lowercase().parse().expect("error on parse");
    input.clear();

    //Calculation
    let free_adults: u32 = no_children / 10;
    let children_cost: f32 = no_children as f32 * CHILD;
    let mut adult_cost: f32 = 0.00;

    if free_adults <= no_adults {
        adult_cost = (no_adults - free_adults) as f32 * ADULT;
    }

    let concession_cost = no_concessions as f32 * CONC;

    let mut total_price = children_cost + adult_cost + concession_cost;
    //Apply discount
    if total_price > 100.0 {
        total_price *= 0.9;
    }

    let postage_cost = if collection == "no" {
        POSTAGE_COST
    } else {
        0.0
    };
    total_price += postage_cost;

    //Print receipt
    println!();
    println!("{no_children} Children Cost: £{children_cost:.2}");
    println!("{no_adults} Adult(s) Cost: £{adult_cost:.2}");
    println!("{no_concessions} Concessions Cost: £{concession_cost:.2}");
    println!("Postage & Packaging: £{postage_cost}");
    println!("--------------");
    println!("Total: {total_price:.2}");
}
