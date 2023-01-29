use std::io;

fn main() {
    let fibonacci: [u128; 2] = [0, 1];
    let n = input_n();
    let result = calculate_fibonacci(n - 1, fibonacci);
    println!("nth Fibonacci number: {result}");
}

fn calculate_fibonacci(n: usize, mut memo: [u128; 2]) -> u128 {
    let mut i = 2;
    let mut result = memo[0] + memo[1];
    while i <= n {
        result = memo[0] + memo[1];
        memo[0] = memo[1];
        memo[1] = result;
        i += 1;
    }
    result
}

fn input_n() -> usize {
    println!("Please input a number between 1 and 187");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Not a valid input");
    match n.trim().parse() {
        Ok(n) => match n {
            1..=187 => n,
            _ => retry_input(),
        },
        Err(_) => retry_input(),
    }
}

fn retry_input() -> usize {
    println!("Not a valid number. Try again!");
    input_n()
}
