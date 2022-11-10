use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play guessing game!");

    let secret_num: u32 = rand::thread_rng().gen_range(1..101);
    println!("Your secret number is {}", secret_num);

    loop {
        println!("Please input your guess.");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        if user_input.trim() == "exit" {
            break;
        }

        let user_input = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed number is {}", user_input);

        match user_input.cpm(&secret_num) {
            Ordering::Less => {
                println!("Too small!")
            }
            Ordering::Greater => {
                println!("Too big!")
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
