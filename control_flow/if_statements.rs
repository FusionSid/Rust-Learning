/* If Statements:

And if statment is used to check if a condition is true and if so it runs some code.
An if statment only runs if the expression/condition is true.
There is also else and else if statments to run if a condition is not true.
*/
fn main() {
    let mut x = 5;

    if x == 5 {
        // This will only run if x = 5 is True.
        println!("x is 5 lol");
    }

    x = 4; // lets change the value of x and see if it works now

    if x == 5 {
        println!("x is 5 (if this runs the computer has gotta 'get good')"); // This will not run since x is not equal to 5
    } else {
        // here i put an else statement
        // This else statement runs if the if condition in the if/else if statement was false
        println!("x is not 5 (sounds like a skill issue ngl)");
    }

    /*
    Now lets do some else if statments
    An else if stament runs if the if statement failed. The program will go through all the else if (if any) and check for those
    If none of those are true then it will run the else block (if there is one) and then end.
    */

    let y = 42;
    if y == 32 {
        println!("y is equal to {}", y);
    } else if y == 42 {
        // This will run because y is equal to 42
        println!("y is equal to {}", y);
    } else {
        println!("y has a skill issue");
    }

    // You can also use if expression in a let statement like this
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number); // since the var condition is true the value of number should be 4
}
