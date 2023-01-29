use std::io;

fn main() {
    let scale = input_scale();
    let temperature = input_temperature();
    let result_temperature = convert(temperature, scale);
    let result_scale = invert_scale(scale);
    println!("{temperature}˚{scale} = {result_temperature}˚{result_scale}");
}

fn convert(input_value: f64, input_scale: char) -> f64 {
    match input_scale {
        'C' => input_value * 1.8 + 32.0,
        'F' => (input_value - 32.0) * 5.0 / 9.0,
        _ => panic!("Not a valid temperature scale!"),
    }
}

fn input_temperature() -> f64 {
    println!("Please input temperature");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid input.");
    match input.trim().parse() {
        Ok(temperature) => temperature,
        Err(_) => retry_input_temperature(),
    }
}

fn input_scale() -> char {
    println!("Please input temperature scale you want to convert from. Possible options: F, C");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid temperature scale.");
    match input.trim().to_uppercase().parse() {
        Ok(scale) => {
            if scale == 'F' || scale == 'C' {
                scale
            } else {
                retry_input_scale()
            }
        }
        Err(_) => retry_input_scale(),
    }
}

fn invert_scale(scale: char) -> char {
    match scale {
        'F' => 'C',
        'C' => 'F',
        _ => panic!("Not a valid temperature scale!"),
    }
}

fn retry_input_scale() -> char {
    println!("Not a valid temperature scale. Try again.");
    input_scale()
}

fn retry_input_temperature() -> f64 {
    println!("Not a valid temperature value. Try again.");
    input_temperature()
}
