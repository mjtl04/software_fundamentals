use std::{io, usize};

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
    println!("\nIn third place: {}", talent_contest[1].name);
    println!("In second place: {}", talent_contest[1].name);
    println!("And the winner is: {}", talent_contest[0].name);
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
        println!("Enter the Cadidate's Number you wish to vote for: ");
        let vote_input = get_input();
        if vote_input == -1 {
            break;
        }
        if vote_input < candidates.len() as i32 {
            candidates[vote_input as usize - 1].votes += 1;
        } else {
            println!("Invalid Vote");
        }
    }
}

fn get_order(candidates: &mut [Candidates; 5]) {
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
