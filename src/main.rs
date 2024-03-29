extern crate rand;

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    print!("The secret number is: {}", secret_number);

    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed: {}", guess);

    match guess.trim().parse::<u32>() {
        Ok(num) => match num.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => print!("Win!"),
        },
        Err(_) => println!("Invalid input!"),
    }
}
