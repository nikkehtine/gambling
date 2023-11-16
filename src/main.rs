// use rand::Rng;
use std::io;

fn main() {
    deposit();
}

fn deposit() -> u32 {
    loop {
        let mut amount = String::new();

        println!("How much $ would you like to deposit?");
        io::stdin()
            .read_line(&mut amount)
            .expect("Could not read line");

        let amount: u32 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if amount > 0 {
            return amount;
        } else {
            println!("Amount must be greater than 0")
        }
    }
}
