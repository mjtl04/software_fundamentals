use std::cmp;

pub fn task1_main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let smallest = recursion_smallest_algorithm(&arr, 0);
    match smallest {
        Some(value) => println!("Smallest: {}", value),
        None => println!("No values"),
    }
}

fn recursion_smallest_algorithm(array: &[u32], index: usize) -> Option<u32> {
    //Smallest in array
    if index < array.len() {
        match recursion_smallest_algorithm(array, index + 1) {
            Some(value) => Some(cmp::min(array[index], value)),
            None => Some(array[index]),
        }
    } else {
        None
    }
}

pub fn task2_main() {
    let arr: [u32; 8] = [1, 2, 3, 4, 6, 7, 9, 10];
    let smallest = recursion_even_algorithm(&arr, 0);
    println!("{}", smallest)
}

fn recursion_even_algorithm(array: &[u32], index: usize) -> u32 {
    if index < array.len() {
        if array[index] % 2 == 0 {
            array[index] + recursion_even_algorithm(array, index + 1)
        } else {
            recursion_even_algorithm(array, index + 1)
        }
    } else {
        0
    }
}
