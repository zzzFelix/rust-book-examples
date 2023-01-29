use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

fn main() {
    let secret_number: u8 = generate_secret_number();

    println!("Guess the secret number between 1 and 100!");
    println!("Please input your guess.");

    loop {
        let guess: u8 = match input_guess() {
            Ok(num) => num,
            Err(_) => continue,
        };
        give_feedback(&guess, &secret_number);
        if is_correct_guess(&guess, &secret_number) {
            break;
        }
    }

    println!("The secret number was {secret_number}");
}

fn generate_secret_number() -> u8 {
    rand::thread_rng().gen_range(1..=100)
}

fn give_feedback(guess: &u8, secret_number: &u8) {
    match &guess.cmp(secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn is_correct_guess(guess: &u8, secret_number: &u8) -> bool {
    guess == secret_number
}

fn input_guess() -> Result<u8, ParseIntError> {
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse()
}
