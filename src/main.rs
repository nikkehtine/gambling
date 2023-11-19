use gambling;

fn main() {
    let balance: u32 = gambling::deposit();

    let mut lines: u32;
    let mut bet: u32;
    let mut total_bet: u32;

    loop {
        lines = gambling::get_number_of_lines();
        bet = gambling::get_bet();
        total_bet = bet * lines;

        if total_bet > balance {
            println!("You can't afford this bet.");
            println!("The total bet is ${total_bet} and your current balance is ${balance}.");
        } else {
            break;
        }
    }

    println!("You are betting ${bet} on {lines} lines. Total bet is ${total_bet}.");

    let slots = gambling::get_slot_machine_spin();
    gambling::print_slot_machine(slots);
}
