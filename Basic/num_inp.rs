use std :: io;

fn main()
{
    let mut number = String :: new;

    print!("Enter a number : ");

    io :: stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    let number: i32 = number.trim().parse().expect("Invalid number");

    println!("Ther Enrtered number is : {}", number);
}