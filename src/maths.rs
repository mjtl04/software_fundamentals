use std::io;

pub fn task1() {
    println!("Cuboid Application");

    let mut input = String::new();

    println!("Length: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error on length input");
    let length: u32 = input.trim().parse().expect("Error on length parsing");
    input.clear();

    println!("Width: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error on width input");
    let width: u32 = input.trim().parse().expect("Error on width parsing");
    input.clear();

    println!("Height: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error on height input");
    let height: u32 = input.trim().parse().expect("Error on height parsing");

    //Calculation
    let volume: u32 = length * width * height;
    let surface_area: u32 = 2 * ((length * width) + (width * height) + (height * length));

    //Output Results
    println!("Volume: {volume}");
    println!("Surface Area: {surface_area}");
}

pub fn task2() {
    const HOUR_IN_MINUTES: f32 = 60.0;
    const CYCLING_CBPM: f32 = 200.0 / HOUR_IN_MINUTES;
    const RUNNING_CBPM: f32 = 475.0 / HOUR_IN_MINUTES;
    const SWIMMING_CBPM: f32 = 275.0 / HOUR_IN_MINUTES;
    const POUND_CALORIE_BURNT: f32 = 3500.0;

    println!("Weight Loss Application: ");

    let mut input = String::new();

    println!("Minutes cycling: ");
    io::stdin()
        .read_line(&mut input)
        .expect("error on cycling input");
    let active_minutes_cycling: u32 = input
        .trim()
        .parse()
        .expect("error on minutes cycled parsing");
    let hours_cycling: u32 = active_minutes_cycling / HOUR_IN_MINUTES as u32;
    let minutes_cycling: u32 = active_minutes_cycling - (hours_cycling * HOUR_IN_MINUTES as u32);
    let cycling_calories: f32 = active_minutes_cycling as f32 * CYCLING_CBPM;
    let cycling_pounds: f32 = cycling_calories / POUND_CALORIE_BURNT;
    input.clear();

    println!("Minutes running: ");
    io::stdin()
        .read_line(&mut input)
        .expect("error on running input");
    let active_minutes_running: u32 = input
        .trim()
        .parse()
        .expect("error on minutes running parsing");
    let hours_running: u32 = active_minutes_running / HOUR_IN_MINUTES as u32;
    let minutes_running: u32 = active_minutes_running - (hours_running * HOUR_IN_MINUTES as u32);
    let running_calories: f32 = active_minutes_running as f32 * RUNNING_CBPM;
    let running_pounds: f32 = running_calories / POUND_CALORIE_BURNT;
    input.clear();

    println!("Minutes swimming: ");
    io::stdin()
        .read_line(&mut input)
        .expect("error on swimming input");
    let active_minutes_swimming: u32 = input
        .trim()
        .parse()
        .expect("error on minutes swimming parsing");
    let hours_swimming: u32 = active_minutes_swimming / HOUR_IN_MINUTES as u32;
    let minutes_swimming: u32 = active_minutes_swimming - (hours_swimming * HOUR_IN_MINUTES as u32);
    let swimming_calories: f32 = active_minutes_swimming as f32 * SWIMMING_CBPM;
    let swimming_pounds: f32 = swimming_calories / POUND_CALORIE_BURNT;

    println!(
        "{:<12} {:>10} {:>16} {:>14}",
        "Activity", "Time Spent", "Calories Burnt", "Pounds Lost"
    );
    println!(
        "{:<12} {:>10} {:>16.2} {:>14.3}",
        "Cycling",
        format!("{}:{:02}", hours_cycling, minutes_cycling),
        cycling_calories,
        cycling_pounds
    );
    println!(
        "{:<12} {:>10} {:>16.2} {:>14.3}",
        "Running",
        format!("{}:{:02}", hours_running, minutes_running),
        running_calories,
        running_pounds
    );
    println!(
        "{:<12} {:>10} {:>16.2} {:>14.3}",
        "Swimming",
        format!("{}:{:02}", hours_swimming, minutes_swimming),
        swimming_calories,
        swimming_pounds
    );
}
