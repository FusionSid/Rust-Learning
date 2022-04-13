/*
Integer - i<size> or u<size>
(i means signed | u for unsigned)

Float - f<size>
*/

pub fn main() {
    let hello = "Hello from data_types.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));

    let age: i16;
    let name: &str;
    let rating: f32;
    let letter: char;
    let sid_is_cool: bool;
    
    age = 10; // integer
    name = "Siddhesh"; // string
    rating = 4.5; // float
    letter = 'a'; // char (single quote)
    sid_is_cool = true; // bool
    
    println!("Age: {}\nName: {}\nRating: {}\nLetter: {}\nSid_is_cool: {}", age, name, rating, letter, sid_is_cool);
    
    // If you want to make it mutable use the mut keyword
    let mut name_2 = "Siddhesh";
    println!("Name: {}", name_2);

    name_2 = "FusionSid";
    println!("Name: {}", name_2);
}
