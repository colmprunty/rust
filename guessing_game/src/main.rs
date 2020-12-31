use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = thread_rng();
    let secret_number = rng.gen_range(1..10);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("please type a number");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
