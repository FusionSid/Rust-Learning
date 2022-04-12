pub fn main() {
    let hello = "Hello from variables.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));

    // variables use the let keyword
    let name = "Siddhesh"; // This is immutible by default so it is a const value

    println!("Name: {}", name);
    
    let mut m_name = "Siddhesh"; // using the mut keyword makes it mutable
    println!("mut Name: {}", m_name);
    
    m_name = "Siddhesh"; // re-asigning the value
    println!("mut Name: {}", m_name);

    // Const values (never really used)
    const ID: i16 = 01; // must specify type
    println!("ID: {}", ID);

    // Assigning multiple values in a line
    let (my_name, my_age) = ("siddhesh", 15);
    println!("my_name: {} | my_age: {}", my_name, my_age);
}

/*
Notes:
Variables hold primitave data or references to data
Variables are immutible by default
Use snake case

*/