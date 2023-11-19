use phf::phf_map;
use rand::{thread_rng, Rng};
use std::io;

const MAX_LINES: u32 = 3;
const MAX_BET: u32 = 100;
const MIN_BET: u32 = 2;

const ROWS: u32 = 3;
const COLS: u32 = 3;

const SYMBOL_COUNT: phf::Map<&'static str, u32> = phf_map! {
    "A" => 2,
    "B" => 4,
    "C" => 6,
    "D" => 8,
};

pub fn get_slot_machine_spin() -> Vec<Vec<&'static str>> {
    let mut all_symbols: Vec<&str> = Vec::new();
    for (symbol, symbol_count) in &SYMBOL_COUNT {
        for _ in 0..*symbol_count {
            all_symbols.push(*symbol);
        }
    }

    let mut columns: Vec<Vec<&str>> = Vec::new();
    for _ in 0..COLS {
        let mut column: Vec<&str> = Vec::new();
        let mut current_symbols: Vec<&str> = all_symbols.clone();
        for _ in 0..ROWS {
            let random_value =
                current_symbols.remove(thread_rng().gen_range(0..current_symbols.len()));
            column.push(random_value);
        }
        columns.push(column);
    }
    return columns;
}

pub fn print_slot_machine(columns: Vec<Vec<&str>>) {
    println!();
    for row in 0..columns[0].len() {
        for (i, column) in columns.iter().enumerate() {
            print!("{}", column[row]);
            if i != columns[0].len() - 1 {
                print!("|");
            } else {
                print!("\n");
            }
        }
    }
    println!();
}

pub fn deposit() -> u32 {
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

pub fn get_number_of_lines() -> u32 {
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

pub fn get_bet() -> u32 {
    loop {
        let mut amount = String::new();

        println!(
            "How much $ would you like to bet? (${} - ${})",
            MIN_BET, MAX_BET
        );
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

        match amount {
            MIN_BET..=MAX_BET => return amount,
            _ => println!("Amount should be between ${MIN_BET} and ${MAX_BET}"),
        }
    }
}
