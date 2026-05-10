0use std::io::{stdin, stdout, Write};

fn main()
{
    let mut name = String::new();

    print!("Enter your name: ");
    
    stdout().flush().expect("Error");

    stdin()
        .read_line(&mut name)
        .expect("Error");

    println!("Hello {}", name.trim());
}