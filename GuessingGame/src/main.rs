//Our library input
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

//Entry point
fn main() {
    println!("Guess the number!");

    //Random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //Infinity loop
    loop{
        println!("{}", "Please input your guess.".blue());

        let mut guess = String::new();

        //User input
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        //Conver the input number from string into integer value
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        

        //Comparing the secret number with user input
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }

}
