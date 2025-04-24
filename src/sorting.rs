use std::{io, task::Wake, usize};

#[derive(Default, Debug)]
struct Candidates {
    name: String,
    votes: u32,
}

impl Candidates {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            votes: 0,
        }
    }
}

pub fn task1_main() {
    let mut talent_contest: [Candidates; 5] = initialise_candidates();
    get_votes(&mut talent_contest);

    println!("\nVote Suummary");
    for value in talent_contest.iter() {
        println!("{} {}", value.name, value.votes)
    }

    get_order(&mut talent_contest);

    //task1 display
    println!("\nIn third place: {}", talent_contest[2].name);
    println!("In second place: {}", talent_contest[1].name);
    println!("And the winner is: {}", talent_contest[0].name);

    //task2 display
    //order_votes(&mut talent_contest);
}

fn initialise_candidates() -> [Candidates; 5] {
    [
        Candidates::new("Ahmed"),
        Candidates::new("Boo"),
        Candidates::new("Celine"),
        Candidates::new("Didi"),
        Candidates::new("Elaine"),
    ]
}

fn get_input() -> i32 {
    let mut numbers = -1;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(value) => {
                numbers = value;
                break;
            }
            Err(_) => {
                println!("Enter a valid number");
            }
        };
    }
    numbers
}

fn get_votes(candidates: &mut [Candidates; 5]) {
    loop {
        println!("Enter the Cadidate's Number you wish to vote for: (-1 to exit) ");
        let vote_input = get_input();
        if vote_input == -1 {
            break;
        }
        if vote_input < candidates.len() as i32 {
            candidates[vote_input as usize].votes += 1;
        } else {
            println!("Invalid Vote");
        }
    }
}

fn get_order(candidates: &mut [Candidates; 5]) {
    //bubble sort
    for upper_index in (1..candidates.len()).rev() {
        let mut swapped = false;
        for index in 0..upper_index {
            if candidates[index].votes < candidates[index + 1].votes {
                candidates.swap(index, index + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn order_votes(candidates: &mut [Candidates; 5]) {
    let first_place_votes = candidates[0].votes;
    let first_place: Vec<&Candidates> = candidates
        .iter()
        .take_while(|c| c.votes == first_place_votes)
        .collect();

    let second_place_votes = candidates[first_place.len()].votes;
    let second_place: Vec<&Candidates> = candidates
        .iter()
        .skip(first_place.len())
        .take_while(|c| c.votes == second_place_votes)
        .collect();

    let third_place_votes = candidates[first_place.len() + second_place.len()].votes;
    let third_place: Vec<&Candidates> = candidates
        .iter()
        .skip(first_place.len() + second_place.len())
        .take_while(|c| c.votes == third_place_votes)
        .collect();

    println!("\nFirst Place: {}", display_order(first_place));
    println!("Second Place: {}", display_order(second_place));
    println!("Third Place: {}", display_order(third_place));
}

fn display_order(arr: Vec<&Candidates>) -> String {
    let names: Vec<&str> = arr.iter().map(|c| c.name.as_str()).collect();
    names.join(" and ")
}
