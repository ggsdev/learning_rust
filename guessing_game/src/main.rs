use rand::{Error, Rng};
use std::cmp::Ordering;
use std::{
    io::{self},
    ops::RangeInclusive,
};

fn main() {
    let range: RangeInclusive<i32> = 1..=10;

    let secret_number: i32 = random_number_generator(range);

    loop {
        println!("Type a number:");
        let guessed_number: Result<i32, &str> = get_guessed_number();

        match guessed_number {
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!\n"),
                Ordering::Greater => println!("Too big!\n"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            },
            Err(_) => {
                println!("Please input a valid number.\n");
                continue;
            }
        }
    }
}

fn random_number_generator(range: RangeInclusive<i32>) -> i32 {
    rand::thread_rng().gen_range(range)
}

fn get_guessed_number() -> Result<i32, &'static str> {
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Failed");

    match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Not a number"),
    }
}

fn test_if() -> i32 {
    if true {
        2
    } else {
        5
    }
}