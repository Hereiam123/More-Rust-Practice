use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
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

    //COnvert from String to numeric i32 guess
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    /*Sort of like a switch statement in other
    languages*/
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
