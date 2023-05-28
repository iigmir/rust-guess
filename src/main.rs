use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess_num() -> u64
{
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u64 = guess.trim().parse().expect("Please type a number!");
    return guess
}

fn main()
{
    // let mut guess = String::new(); // mutable
    // let apples = 5; // immutable
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let guess = guess_num();
    println!("You guessed: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
