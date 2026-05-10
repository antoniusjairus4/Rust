use std::io;
use std::io::Write;

fn main()
{
    let mut number = String::new();

    print!("Enter a number : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    let number: i32 = number.trim().parse().expect("Invalid number");

    println!("The Entered number is : {}", number);
}