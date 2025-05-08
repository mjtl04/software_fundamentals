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

    sort_votes(&mut talent_contest);

    //task1 display
    // println!("\nIn third place: {}", talent_contest[2].name);
    // println!("In second place: {}", talent_contest[1].name);
    // println!("And the winner is: {}", talent_contest[0].name);

    //task2 display
    let first = order_votes("First", 0, &mut talent_contest);
    let second = order_votes("Second", first, &mut talent_contest);
    order_votes("Third", second, &mut talent_contest);
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
        } else if vote_input < -1 {
            println!("Enter a valid vote");
        } else if vote_input < candidates.len() as i32 {
            candidates[vote_input as usize].votes += 1;
        } else {
            println!("Invalid Vote");
        }
    }
}

fn sort_votes(candidates: &mut [Candidates; 5]) {
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

fn order_votes(leaderboard: &str, position: usize, candidates: &mut [Candidates; 5]) -> usize {
    let value = candidates[position].votes;

    if value != 0 {
        print!("\n{leaderboard}: ");
        for num in position..candidates.iter().len() {
            if candidates[num].votes >= value {
                print!("{}, ", candidates[num].name)
            } else {
                return num;
            }
        }
    }
    0_usize
}
