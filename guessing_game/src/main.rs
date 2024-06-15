use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number game");
    let secret = rand::thread_rng().gen_range(1..=100);
    //println!("Secret Number is {secret}");
    loop {
        println!("Enter your number : ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        println!("You guessed {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
