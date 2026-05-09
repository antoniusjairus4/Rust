use std :: io :: stdin;

fn main()
{
    let mut name = String::new();

    println!("Enter your name:");

    stdin()
        .read_line(&mut name)
        .expect("Error");

    println!("Hello {}", name.trim());
}