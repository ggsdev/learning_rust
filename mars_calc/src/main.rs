use std::io;

fn main() {
    println!("Input your weight:");

    let mut input: String = String::new();

    let _ = io::stdin().read_line(&mut input);

    let weight: f32 = input.trim().parse().unwrap();

    let result: f32 = calculate_weight_on_mars(weight);

    println!("Weight on Mars: {}kg", result)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
