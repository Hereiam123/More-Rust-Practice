use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);
    //Infinite loop to allow multiple guesses
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        /*Return type checked by 'expect',
        checking for Ok or Err returned result.
        If Ok - Value is returned
        If Err - Error returned,
        program stops and error message displayed*/
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*Convert from String to numeric u32 guess.
        Type required for parse of string*/
        //.parse returns a 'Result' type, so 'expect' is needed
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        /*Since above line doesn't stop invalid inputs
        We can unpack the returned 'Result' type, check the match,
        and return the value, or go back to the beginning of the loop
        if there is an error, to prompt the user again*/
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        /*Sort of like a switch statement in other
        languages*/
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //Exit on win
                break;
            }
        }
    }
}
