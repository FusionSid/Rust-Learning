mod calc;
mod if_else;
mod structs;
mod printing;
mod variables;
mod date_types;

fn main() {
    printing::main();
    if_else::main();
    variables::main();
    date_types::main();

    let hello = "Hello from calc.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));

    let num1 = 10;
    let num2 = 2;

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
