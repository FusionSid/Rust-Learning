mod calc;
mod if_else;
mod structs;
mod printing;
mod variables;
mod date_types;

use std::io;

fn main() {
    printing::main();
    if_else::main();
    variables::main();
    date_types::main();

    let hello = "Hello from calc.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));

    println!("Enter the first number");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: i32 = num1.trim().parse().expect("Please type a number!");

    
    println!("Enter the second number");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: i32 = num2.trim().parse().expect("Please type a number!");


    let mut output = calc::add(&num1, &num2);
    println!("Adding: {}", output);

    output = calc::subtract(&num1, &num2);
    println!("Subtracting: {}", output);

    output = calc::multiply(&num1, &num2);
    println!("Multiply: {}", output);

    output = calc::divide(&num1, &num2);
    println!("Divide: {}", output);

    let user = structs::run(); // Creating user
    println!("Username: {} (from main.rs)", user.username); // printing user.username

    let rect = structs::Rect{
        width: 43,
        height: 78
    };

    println!("Rect width: {}\nRect height: {}\nRect area: {}", rect.width, rect.height, rect.width*rect.height);
}
