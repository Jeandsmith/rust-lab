use std::io;

fn main() {
    println!("Guess The Number Game");

    println!("Input Your Guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess is: {guess}");
}
