use std::io;

pub fn task1() {
    println!("Enter 10 integers: ");
    let mut arr: [u32; 10] = [0; 10];
    let mut smallest_index = 0;

    for i in 0..10 {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("error on input");

            match input.trim().parse::<u32>() {
                Ok(value) => {
                    arr[i] = value;
                    break;
                }
                Err(_) => println!("Invalid Input. Enter a number: "),
            }
        }
    }

    for (index, &item) in arr[1..].iter().enumerate() {
        if item < arr[smallest_index] {
            smallest_index = index + 1;
        }
    }

    println!(
        "Smallest value: {} at index {smallest_index}",
        arr[smallest_index]
    )
}

pub fn task2() {
    let mut input = String::new();

    let mut students: Vec<String> = Vec::<String>::new();
    let mut grades: Vec<u32> = Vec::<u32>::new();

    println!("Number of students: ");
    io::stdin().read_line(&mut input).expect("error on input");

    let student_count: u32 = match input.trim().parse() {
        Ok(value) if value > 0 => value,
        _ => {
            println!("No students entered. Leaving");
            return;
        }
    };

    for i in 0..student_count {
        println!("Student Name {i}: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("error on input");
        students.push(input.trim().to_string());

        println!("Student Grade: ");
        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("error on input");
            match input.trim().parse::<u32>() {
                Ok(value) => {
                    grades.push(value);
                    break;
                }
                Err(_) => println!("Enter a valid grade: "),
            }
        }
    }

    let sum_grades: u32 = grades.iter().sum();
    let average_grade: f32 = sum_grades as f32 / student_count as f32;

    println!("\nAverage: {average_grade:.2}");
    println!("Students above average: ");

    for (i, &grade) in grades.iter().enumerate() {
        if grade as f32 > average_grade {
            println!("{}, {grade}", students[i]);
        }
    }
}
