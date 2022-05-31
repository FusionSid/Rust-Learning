/* 
Scalar Types
A scalar type represents a single value. 
Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 
*/
fn main() {
    immutable();
    mutable();
}

/* 
Immutable variables
In rust variables are immutible by default
*/
fn immutable() {
    println!("Immutable variables:\n--------------------");
    /* Integers

    Signed or Unsigned:
        i8	  u8
        i16	  u16
        i32	  u32
        i64	  u64
        i128  u128
        isize usize

    Unsigned are the same a integers but no negative numbers
    */
    let a = 69; // Let the compiler figure out the type
    let b: u32 = 69; // Explicitly define the type
    println!("a: {}\nb: {}", a, b);

    /* Floating-Points

    Single and double precision:
        f32 or f64
    */
    let c = 42.69;
    let d: f32 = 42.69;
    println!("c: {}\nd: {}", c, d);
    
    // Booleans: true or false values
    let e = true;
    let f: bool = false;
    println!("e: {}\nf: {}", e, f);

    // Chars: a single character
    let g = 'a';
    let h: char = 'c';

    println!("g: {}\nh: {}", g, h);
}

/* 
Mutable Variables

Since variables are immutible by default 
This means the variable cant be reasigned/changed

To make a variable mutable we use the "mut" keyword
*/
fn mutable() {
    println!("\n\nMutable variables:\n------------------");
    let _a = 12345; // This variable cant be changed
    /* If I type this: 
    a = 54321;
    I will get an error*/

    let mut b = 6942; // This variable is mutable
    println!("b: {}", b);

    // It can be changed like this:
    b = 4269; // Note: we dont use the let keyword cause we are not reasigning it
    println!("b: {}", b);
}