/*
Integer - i<size> or u<size>

(i means signed | u for unsigned)
*/

pub fn main() {
    let hello = "Hello from data_types.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));

    // Unsigned
    let age: i16;
    let rating: f32;
    let letter: char;
    let sid_is_cool: bool;

    // Signed
    age = 10; // integer
    rating = 4.5; // float
    letter = 'a'; // char (single quote)
    sid_is_cool = true; // bool
    
    println!("Age: {}\nRating: {}\nLetter: {}\nSid_is_cool: {}", age, rating, letter, sid_is_cool);
    
    // If you want to make it mutable use the mut keyword
    let mut name = String::new(); // i couldnt find how to do this so i think this is how
    name.push_str("Siddhesh"); // string (double quote)
    println!("Name: {}", name)
}
