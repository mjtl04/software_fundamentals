use std::io;

pub fn task1() {
    println!("Enter 10 integers: ");
    let mut arr: [u32; 10] = [0; 10];

    let mut input = String::new();

    for i in 0..10 {
        input.clear();
        io::stdin().read_line(&mut input).expect("error on input");

        arr[i] = input.trim().parse().expect("error on parse");
    }

    let mut smallest: u32 = arr[0];
    let mut index: usize = 0;

    for (i, &val) in arr.iter().enumerate() {
        if val < smallest {
            smallest = val;
            index = i;
        }
    }

    println!("Smallest value: {smallest} at index {index}")
}
