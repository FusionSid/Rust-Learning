/*
Compound Types
Compound types can group multiple values into one type. 
Rust has two primitive compound types: tuples and arrays.
*/
fn main() {
    /* 
    Tuples:

    Tuples are a way to group together a varietry of variables
    Tuples have a fixed length so once declared they canot be grow/shrink size
    */
    
    /* 
    Here we make a variable called tup and add 3 values in it
    We also add type annotations which is not neccacery
    */
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    /* 
    You can also deconstruct a tuple into values
    Here i asign the variables: x, y and x to the values in the tup
    */
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    //To get an element from the tuple we use a period followed by the index.
    println!("X is equal to: {}", tup.0);


    /*
    Arrays:

    Arrays are another way to store a collection of values.
    Unlike tuples arrays must have values of the same type
    Arrays are located in the stack rather than the heap
    Also unlike other languages arrays in rust are a fixed length
    If that data needs to grow/shrink use a vector instead
    */
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    // You can also anotate the type and length of the array
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // To get an element from an array we use a square brackets with the index inside
    let c = months[0];
    println!("The first months of the year is {}", c);


}