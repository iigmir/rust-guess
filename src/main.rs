use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess_num() -> u64
{
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u64 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    return guess
}

fn comparing_number(secret_number: u64, guessing_number: u64) -> String
{
    match guessing_number.cmp(&secret_number) {
        Ordering::Less => "Too small!".to_string(),
        Ordering::Greater => "Too big!".to_string(),
        Ordering::Equal => "You win!".to_string()
    }
}

fn main()
{
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let guess = guess_num();
        let message = comparing_number( secret_number, guess );
        println!("You guessed: {}", guess);
        println!( "{}", message );
        if guess == secret_number {
            break;
        }
    }
}
