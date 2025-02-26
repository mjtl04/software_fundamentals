use std::io;

pub fn task1(){
    println!("Cuboid Application");

    let mut input = String::new();

    println!("Length: ");
    io::stdin().read_line(& mut input).expect("Error on input");
    let length:u32 = input.trim().parse().expect("Error on input parsing");
    input.clear();

    println!("Width: ");
    io::stdin().read_line(& mut input).expect("Error on input");
    let width:u32 = input.trim().parse().expect("Error on input parsing");
    input.clear();

    println!("Height: ");
    io::stdin().read_line(& mut input).expect("Error on input");
    let height:u32 = input.trim().parse().expect("Error on input parsing");
    input.clear();

    let volume:u32 = length * width * height;
    let surface_area:u32 = 2 * ((length * width) + (width * height) + (height * length));

    println!("Volume: {volume}");
    println!("Surface Area: {surface_area}");

}
