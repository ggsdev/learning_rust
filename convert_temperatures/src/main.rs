use std::io;

fn main() {
    println!("Input a temperature in Celsius.");

    let input = loop {
        match get_temperature_input() {
            Ok(temp) =>break temp,
            Err(e) => println!("{e}"),
        }
    };

    let result = convert_to_fahrenheit(input);

    println!("result: {result}");
}

fn get_temperature_input() -> Result<f32, String>{

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error");

    input
        .trim()
        .parse()
        .map_err(|_| "Please enter a valid number.".to_string())
}

fn convert_to_fahrenheit(temperature: f32) -> f32{

    (temperature * 9.0 / 5.0) + 32.0
}