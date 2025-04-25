use std::io;

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
    // println!("\nIn third place: {}", talent_contest[2].name);
    // println!("In second place: {}", talent_contest[1].name);
    // println!("And the winner is: {}", talent_contest[0].name);

    //task2 display
    order_votes(&mut talent_contest);
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
    if first_place_votes != 0 {
        // First place
        let first_place: Vec<&Candidates> = candidates
            .iter()
            .take_while(|c| c.votes == first_place_votes)
            .collect();

        let mut next_index = first_place.len();

        // Second place
        let second_place: Vec<&Candidates> = if let Some(candidate) = candidates.get(next_index) {
            let second_place_votes = candidate.votes;
            if second_place_votes != 0 {
                let v: Vec<&Candidates> = candidates
                    .iter()
                    .skip(next_index)
                    .take_while(|c| c.votes == second_place_votes)
                    .collect();
                next_index += v.len();
                v
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Third place
        let third_place: Vec<&Candidates> = if let Some(candidate) = candidates.get(next_index) {
            let third_place_votes = candidate.votes;
            if third_place_votes != 0 {
                candidates
                    .iter()
                    .skip(next_index)
                    .take_while(|c| c.votes == third_place_votes)
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        if !third_place.is_empty() {
            println!("\nThird Place: {}", display_order(third_place));
        }
        if !second_place.is_empty() {
            println!("Second Place: {}", display_order(second_place));
        }
        println!("First Place: {}", display_order(first_place));
    } else {
        println!("\nNo votes entered");
    }
}

fn display_order(arr: Vec<&Candidates>) -> String {
    let names: Vec<&str> = arr.iter().map(|c| c.name.as_str()).collect();
    names.join(" and ")
}
