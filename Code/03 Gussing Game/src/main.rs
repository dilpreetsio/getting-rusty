use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries: i32 = 0;

    loop {
        println!("Please enter the number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // type casting using shadowing variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // old pattern
        // if (secret_number == guess) {
        //     println!("You win!");
        // } else {
        //     println!("You lose!");
        // }

        // Rust pattern
        // match picks the arm that satisfy the condition and executes it
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        tries += 1;
    }
    println!("It took you {} tries to guess!", tries);
}
