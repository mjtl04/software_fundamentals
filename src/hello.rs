use std::io;

fn main() {

            println!("hello from new HelloClass");
            let mut name = String::new();
            
            io::stdin().read_line(&mut name);
            println!("Hello {name}")
            
            

}

