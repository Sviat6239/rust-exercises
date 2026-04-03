//Our library input
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//Entry point
fn main() {
    println!("Guess the number!");

    //Random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    //User input
    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    //Conver the input number from string into integer value
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
      

    //Comparing the secret number with user input
    match guess.cmp(&secret_number){
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
