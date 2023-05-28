use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main()
{
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable
    // let apples = 5; // immutable
    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
