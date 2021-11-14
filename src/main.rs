use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    guess_number();
}
/**
 * @brief Guess the number
 * @return void
 * @author Leo
 */
fn guess_number() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
