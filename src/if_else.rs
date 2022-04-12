pub fn main() {
    let hello = "Hello from if_else.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));
 
    let number = 3; // Makes an int var
    
    println!("Number: {}", number);

    if number < 5 {
        println!("condition (number < 5) was true");
    } else {
        println!("condition (number < 5) was false");
    }
}