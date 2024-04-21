use std::cmp::Ordering;
use std::io;
use std::io::{Read, Write};
use std::num::ParseIntError;
use rand::Rng;

trait MyTrait {}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let a = 1576;
        println!("a = {}", a);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue
        };


        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                print!("Press any key to exit...");
                io::stdout().flush().unwrap();
                let _ = io::stdin().read(&mut [0u8]).unwrap();
                break;
            }
        }
    }
}
