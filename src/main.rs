use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play guessing game!");

    let secret_num: u32 = rand::thread_rng().gen_range(1..101);
    println!("Your secret number is {}", secret_num);

    let mut max_number: u32 = 100;
    let mut min_number: u32 = 1;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        if guess.trim() == "exit" {
            break;
        }

        let guess_num: u32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed number is {}", guess_num);

        match guess_num.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is {}", secret_num);
                break;
            }
            Ordering::Greater => {
                if guess_num <= max_number {
                    max_number = guess_num
                };

                println!("Too big!");
                println!("The secret number is between {} and {}", min_number, max_number);
            }
            Ordering::Less => {
                if guess_num >= min_number {
                    min_number = guess_num
                };

                println!("Too small!");
                println!("The secret number is between {} and {}", min_number, max_number);
            }
        }
    }
}
