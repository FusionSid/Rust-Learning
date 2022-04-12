pub fn main() {
    let hello = "Hello from printing.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));
    // Printing hello world
    println!("Hello, world!");

    // Formatting
    println!("Hello {}", "Siddhesh");

    // Positional Args
    println!("Hello {1} my name is {0}", "Sid", "Siddhesh");
    
    // Named Args
    println!("Hello {name}", name="Siddhesh");

    // Placeholder Traits
    println!("Binary: {:b} | Hex: {:x} | Octal: {:o}", 10, 10, 10);

    // Math
    println!("5 + 5 = {}", 5 + 5);
}