/*
Functions:

Functions in rust use snake_case just like python :)
*/

fn main() {
    hello_world();
    add(6, 9);
    let result = multiply(4, 2);
    println!("Result: {}", result)
}

// To define a function we type "fn" then the function name
fn hello_world() {
    println!("Hello, world!"); // This function prints hello world and returns nothing
}

// Function with parameters
// to add parameters to a function we put their names inside the function's brackets    
fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}

// Function with return things
// To make a function return something we must anotate the return type using an arrow (->) just like python
// We can either use the return keyword or the last value in the function will be returned
fn multiply(x: i32, y: i32) -> i32 {
    return x*y; // Here we return x * y
}

fn multiply2(x: i32, y: i32) -> i32 {
    x*y // Here we return x * y without using the return keyword
    // Note that there is no semi colon at the end 
}