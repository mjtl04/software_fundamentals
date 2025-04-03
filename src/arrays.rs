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

    for index in 0..arr.len() {
        if arr[index] < arr[smallest_index] {
            smallest_index = index;
        }
    }

    println!(
        "Smallest value: {} at index {smallest_index}",
        arr[smallest_index]
    )
}

pub fn task2() {
    //Needs improving
    let mut input = String::new();

    let mut students_number: u32 = 0;
    let mut students: Vec<String> = Vec::<String>::new();
    let mut grades: Vec<u32> = Vec::<u32>::new();

    println!("Number of students: ");
    io::stdin().read_line(&mut input).expect("error on input");

    students_number = input.trim().parse().expect("error on parse");

    if students_number == 0 {
        println!("No students to process.");
        return;
    }

    for i in 0..students_number {
        println!("Student Name {i}: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("error on input");
        students.push(input.trim().to_string());

        println!("Student Grade: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("error on input");
        grades.push(input.trim().parse().expect("error on parse"));
    }

    let mut sum_grades: u32 = 0;

    for i in &grades {
        sum_grades += i;
    }

    let average_grade: f32 = sum_grades as f32 / students_number as f32;

    println!();
    println!("Average: {average_grade:.2}");
    println!();
    println!("Students above average: ");

    for (i, &grade) in grades.iter().enumerate() {
        if grade as f32 > average_grade {
            println!("{}, {grade}", students[i]);
        }
    }
}
