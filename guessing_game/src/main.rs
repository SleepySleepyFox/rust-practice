use std::io;
use rand::{random_range};
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let mut is_playing = true;
    let secret_number = random_range(1..=100);
    while is_playing{
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");


        match guess.cmp(&secret_number) {
            Ordering::Less =>{
                 println!("Too Small");
                 println!("Try again")
            },
            Ordering::Equal => {
                println!("You win!");
                is_playing = false
            },
            Ordering::Greater =>{
                 println!("Too Big");
                 println!("Try again")
            }
        }
    }

}
