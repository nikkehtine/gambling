use gambling;
use std::io;

fn main() {
    let mut balance: u32 = gambling::deposit();
    loop {
        println!("Your current balance is ${balance}");
        println!("Press Enter to play, or q to quit");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Could not read line");
        match option.trim() {
            "" => {
                balance = balance.saturating_add_signed(spin(&balance));
            }
            "q" => {
                break;
            }
            _ => {
                let _ = (); // do nothing
            }
        };
    }
    println!("You left with ${balance}");
}

fn spin(balance: &u32) -> i32 {
    let mut lines: u32;
    let mut bet: u32;
    let mut total_bet: u32;

    loop {
        lines = gambling::get_number_of_lines();
        bet = gambling::get_bet();
        total_bet = bet * lines;

        if &total_bet > balance {
            println!("You can't afford this bet.");
            println!("The total bet is ${total_bet} and your current balance is ${balance}.");
        } else {
            break;
        }
    }

    println!("You are betting ${bet} on {lines} lines. Total bet is ${total_bet}.");

    let slots = gambling::get_slot_machine_spin();
    gambling::print_slot_machine(&slots);

    let (winnings, winning_lines) = gambling::check_winnings(&slots, lines, bet);
    println!("You won ${winnings}");
    if !winning_lines.is_empty() {
        println!("You won on lines {:?}", &winning_lines);
    }
    return (winnings - total_bet) as i32;
}
