use rug::Integer;
use std::io;

fn main() {
    let fibonacci: [Integer; 2] = [Integer::from(0), Integer::from(1)];
    let n = input_n();
    let result = calculate_fibonacci(n, fibonacci);
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
    println!("Please input a number between 0 and 1000000");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Not a valid input");
    match n.trim().parse() {
        Ok(n) => match n {
            0..=1000000 => n,
            _ => retry_input(),
        },
        Err(_) => retry_input(),
    }
}

fn retry_input() -> usize {
    println!("Not a valid number. Try again!");
    input_n()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_fibonacci() {
        let n = 0;
        let result = handle_edge_cases(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn first_fibonacci() {
        let n = 1;
        let result = handle_edge_cases(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn fifth_fibonacci() {
        let memo: [Integer; 2] = [Integer::from(0), Integer::from(1)];
        let n = 5;
        let result = handle_default_cases(n, memo);
        assert_eq!(result, 5);
    }

    #[test]
    fn millionth_fibonacci() {
        let memo: [Integer; 2] = [Integer::from(0), Integer::from(1)];
        let n = 1000000;
        let result = handle_default_cases(n, memo);
        let result_as_string = &result.to_string_radix(10)[..10];
        assert_eq!(result_as_string, String::from("1953282128"));
    }
}
