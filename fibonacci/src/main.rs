use rug::Integer;
use std::io;

fn main() {
    let fibonacci: [Integer; 2] = [Integer::from(0), Integer::from(1)];
    let n = input_n();
    let result = calculate_fibonacci(n - 1, fibonacci);
    println!("nth Fibonacci number: {result}");
}

fn calculate_fibonacci(n: usize, memo: [Integer; 2]) -> Integer {
    match n {
        0..=1 => handle_edge_cases(n),
        n => handle_default_cases(n, memo),
    }
}

fn handle_default_cases(n: usize, mut memo: [Integer; 2]) -> Integer {
    let mut i = 2;
    let mut result: Integer = (&memo[0] + &memo[1]).into();
    while i <= n {
        result = (&memo[0] + &memo[1]).into();
        memo[0] = memo[1].clone();
        memo[1] = result.clone();
        i += 1;
    }
    result
}

fn handle_edge_cases(n: usize) -> Integer {
    match n {
        0 => Integer::from(0),
        1 => Integer::from(1),
        _ => panic!("This cannot happen"),
    }
}

fn input_n() -> usize {
    println!("Please input a number between 1 and 1000000000");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Not a valid input");
    match n.trim().parse() {
        Ok(n) => match n {
            1..=1000000000 => n,
            _ => retry_input(),
        },
        Err(_) => retry_input(),
    }
}

fn retry_input() -> usize {
    println!("Not a valid number. Try again!");
    input_n()
}
