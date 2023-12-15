// bring io into scope from std
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    // print output using println! macro
    println!("Guess the number!");

    // set variable secret_number to the result of the rand function
    // with the range of 1 to or equal too 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // a loop so a user can try multiple times
    loop {   
        println!("Please input your guess.");

        // create a new mutible varible with the new() function of String
        let mut guess = String::new();

        // call the stdin() function of the io library
        io::stdin()
            // read_line is a function that takes in a reference to a valible
            // that stores the user stdin
            .read_line(&mut guess)
            // expect will catch a "Result" or enum the Result will either be
            // Ok or Err -> Err will contain additional information about this problem
            // expect is a method of the "Result" type which is what is returned by read_line
            // // expect will crash with the below message if Result returns Err
            .expect("Failed to read line");

        // Variable shadowing
        // This is a new variable called guess with the same name -> new memory
        // when the user inputs information they send a newline char (\n)
        // this char needs to be stripped before parsing to u32
        // The colon after the variable name denotes type annotation
        let guess: u32 = match guess.trim().parse() {
            // instead of just crashing on invalid input
            // lets discard it and let the user keep playing
            // This is a match expression for the Result
            Ok(num) => num,
            Err(_) => continue,
        };

        // use println macro to print out the guess
        // Think of {} as little crab pincers that hold a value in place.
        println!("You guessed: {guess}");

        // The ordering type is an enum that has Less, Greater, Equal
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // end program on successful guess
                println!("You win!");
                break;
            }
        }
    }
}
