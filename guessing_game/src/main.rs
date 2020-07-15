use std::io;

fn main() {
    println!("Guess the number!");
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
    println!("You guessed: {}", guess);
}
