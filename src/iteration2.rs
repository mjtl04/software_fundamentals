use rand::Rng;

pub fn task1(){
    println!("matching die");

    let mut die1: u32 = 0;
    let mut die2: u32 = 1;

    let mut rng = rand::thread_rng();
    let mut counter = 0;

    while die1 != die2 {;
        println!("Rolling die..");

        die1 = rng.gen_range(1..=6);
        die2 = rng.gen_range(1..=6);
        counter += 1;
    }
    
    println!("It took {counter} to roll {die1} on both die");

}
