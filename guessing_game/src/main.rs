use std::io;
use std::cmp::Ordering;
use std::num::ParseIntError;
use rand::Rng;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number between 1 and 100!");
    println!("Please input your guess.");

    loop {
        let guess: u8 = match input_guess() {
            Ok(num) => num,
            Err(_) => continue
        };
        let correct_guess = compare_numbers(&guess, &secret_number);
        if correct_guess == true {
            break;
        }
    }

    println!("The secret number was {secret_number}");
}

fn compare_numbers(guess: &u8, secret_number: &u8) -> bool {
    match &guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    return false;
}

fn input_guess() -> Result<u8, ParseIntError> {
    let mut guess: String = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    guess.trim().parse()
}
