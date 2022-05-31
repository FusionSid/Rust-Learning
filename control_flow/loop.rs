/* Loop in rust

In rust we have this cool block called loop. Its essentialy a while true loop.
It will run forever until we tell it to stop.
There are 2 keyword we can use inside a loop:
to break out of a loop we use "break;" and to continue to the next iteration we use "continue;"
*/

fn main() {
    let mut count = 0;
    'counting_up: loop {
        // here i give the outer loop a name of counting_up
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            // this inner loop will loop until remaining is equal to 9
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // the outer loop will run until count is equal to 2
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    /*
    in total the main loop ran 3 times and the output was:
        count = 0
        remaining = 10
        remaining = 9
        count = 1
        remaining = 10
        remaining = 9
        count = 2
        remaining = 10
        End count = 2
    */
    println!("End count = {}", count);

    // Another cool thing you can do is return values from loops when you break

    let mut number = 5;
    let mut count = 0;

    let value = loop {
        if count >= 10 {
            break number; // If number reaches ten or higher it will break out of the loop and return the variable "number"
        }
        println!("number: {}, count: {}", number, count);

        number *= 5; // Each iteration of the loop number = number * 5
        count += 1;
    }; // value should be equal to 48828125
    println!("value: {}", value);
}

/*
Since this function isnt being used when i compile the code i get a compiler warning
To supress that i added the allow dead code comment thing and it make the compiler not warn me :)
*/
#[allow(dead_code)]
fn again() {
    // i wont run this in the main method cause its infinite
    loop {
        println!("again!");
    } // this goes for ever and ever until i tell it to
}
