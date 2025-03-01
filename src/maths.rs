use std::io;

const HOUR_IN_MINUTES: f32 = 60.0;
const CYCLING_CBPM: f32 = 200.0 / HOUR_IN_MINUTES;
const RUNNING_CBPM: f32 = 475.0 / HOUR_IN_MINUTES;
const SWIMMING_CBPM: f32 = 275.0 / HOUR_IN_MINUTES;

pub fn task1() {
    println!("Cuboid Application");

    let mut input = String::new();

    println!("Length: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let length: u32 = input.trim().parse().expect("Error on input parsing");
    input.clear();

    println!("Width: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let width: u32 = input.trim().parse().expect("Error on input parsing");
    input.clear();

    println!("Height: ");
    io::stdin().read_line(&mut input).expect("Error on input");
    let height: u32 = input.trim().parse().expect("Error on input parsing");
    input.clear();

    let volume: u32 = length * width * height;
    let surface_area: u32 = 2 * ((length * width) + (width * height) + (height * length));

    println!("Volume: {volume}");
    println!("Surface Area: {surface_area}");
}

pub fn task2() {
    println!("Weight Loss Application: ");

    let mut input = String::new();

    println!("minutes cycling: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let active_minutes_cycling: u32 = input.trim().parse().expect("error on input parsing");
    let hours_cycling: u32 = active_minutes_cycling / HOUR_IN_MINUTES as u32;
    let minutes_cycling: u32 = active_minutes_cycling - (hours_cycling * HOUR_IN_MINUTES as u32);
    let cycling_calories: f32 = active_minutes_cycling as f32 * CYCLING_CBPM;
    let cycling_pounds: f32 = cycling_calories / 3500 as f32;
    input.clear();

    println!("minutes running: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let active_minutes_running: u32 = input.trim().parse().expect("error on input parsing");
    let hours_running: u32 = active_minutes_running / HOUR_IN_MINUTES as u32;
    let minutes_running: u32 = active_minutes_running - (hours_running * HOUR_IN_MINUTES as u32);
    let running_calories: f32 = active_minutes_running as f32 * RUNNING_CBPM;
    let running_pounds: f32 = running_calories / 3500 as f32;
    input.clear();

    println!("minutes swimming: ");
    io::stdin().read_line(&mut input).expect("error on input");
    let active_minutes_swimming: u32 = input.trim().parse().expect("error on input parsing");
    let hours_swimming: u32 = active_minutes_swimming / HOUR_IN_MINUTES as u32;
    let minutes_swimming: u32 = active_minutes_swimming - (hours_swimming * HOUR_IN_MINUTES as u32);
    let swimming_calories: f32 = active_minutes_swimming as f32 * SWIMMING_CBPM;
    let swimming_pounds: f32 = swimming_calories / 3500 as f32;
    input.clear();

    println!("Activity      Time Spent  Calories Burnt  Pounds Lost");
    println!("Cycling       {hours_cycling}:{minutes_cycling}        {cycling_calories}            {cycling_pounds:.3}");
    println!("Running       {hours_running}:{minutes_running}        {running_calories}            {running_pounds:.3}");
    println!("Swimming      {hours_swimming}:{minutes_swimming}      {swimming_calories}           {swimming_pounds:.3}");
}
