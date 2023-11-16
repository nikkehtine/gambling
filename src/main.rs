// use rand::Rng;
use std::io;

const MAX_LINES: u32 = 3;

fn main() {
    let balance: u32 = deposit();
    let lines = get_number_of_lines();
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

fn get_number_of_lines() -> u32 {
    loop {
        let mut lines = String::new();

        println!("How many lines do you want to bet on? (1-{MAX_LINES})");
        io::stdin()
            .read_line(&mut lines)
            .expect("Could not read line");

        let lines: u32 = match lines.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match lines {
            1..=MAX_LINES => return lines,
            _ => println!("Amount not in range"),
        }
    }
}
