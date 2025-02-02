// library dependencies
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // debug only
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // mutable variable
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        println!("Your guess: {guess}");
    
        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
