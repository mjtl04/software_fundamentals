use std::cmp;

pub fn task1_main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let smallest = recursion_algorithm(&arr, 0);
    match smallest {
        Some(value) => println!("Smallest: {}", value),
        None => println!("No values"),
    }
}

fn recursion_algorithm(array: &[u32], index: usize) -> Option<u32> {
    //Smallest in array
    if index < array.len() {
        match recursion_algorithm(array, index + 1) {
            Some(value) => Some(cmp::min(array[index], value)),
            None => Some(array[index]),
        }
    } else {
        None
    }
}
