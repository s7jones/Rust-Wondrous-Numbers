use std::io::prelude::*;
use std::io;

fn main() {
    println!("Program modes");
    println!("\t1 - Calculate sequence for an individual number, n.");
    println!("\t2 - Calculate sequence for an individual number, n (parity).");
    println!("\t3 - Reverse tree calcution upto n steps.");

    let mut menu = String::new();

    print!("\nMenu choice:\t");

    io::stdout().flush().ok().expect("didn't flush stdout");

    io::stdin()
        .read_line(&mut menu)
        .ok()
        .expect("failed to read line");

    let menu: u8 = menu.trim()
        .parse()
        .ok()
        .expect("failed to convert to number");

    match menu {
        1 => {
            println!("\n--- CALCULATE COLLATZ SEQUENCE FOR A SINGLE NUMBER ---\n");
            print!("Please choose a number:\t");

            io::stdout().flush().ok().expect("didn't flush stdout");

            let mut number = String::new();

            io::stdin()
                .read_line(&mut number)
                .ok()
                .expect("failed to read line");

            let mut number: u64 = number.trim()
                .parse()
                .ok()
                .expect("failed to convert to number");

            println!("You chose: {}", number);

            let mut n = 0;
            loop {
                println!("Iteration {}, number: {}", n, number);
                if number == 1 {
                    break;
                }
                if number % 2 == 1 {
                    number = 3 * number + 1;
                } else {
                    number = number / 2;
                }
                n += 1;
            }
        }
        2 => {
            println!("\n--- CALCULATE COLLATZ SEQUENCE FOR A SINGLE NUMBER (PARITY) ---\n");
            print!("Please choose a number:\t");

            io::stdout().flush().ok().expect("didn't flush stdout");

            let mut number = String::new();

            io::stdin()
                .read_line(&mut number)
                .ok()
                .expect("failed to read line");

            let mut number: u64 = number.trim()
                .parse()
                .ok()
                .expect("failed to convert to number");

            println!("You chose: {}", number);

            let mut n = 0;
            let mut parity_sequence = String::new();

            loop {
                let parity = number % 2;
                parity_sequence.push_str(&parity.to_string());
                println!("Iteration {}, number: {}, parity: {}", n, number, parity);
                if number == 1 {
                    break;
                }
                if parity == 1 {
                    number = (3 * number + 1) / 2;
                } else {
                    number = number / 2;
                }
                n += 1;
            }
            println!("Parity sequence: {}.", parity_sequence);
        }
        3 => {
            println!("\n--- CALCULATE REVERSE COLLATZ SEQUENCE FOR A NUMBER OF STEPS ---\n");

        }
        _ => {
            println!("Choose a correct choice from the menu. Exiting...");
        }
    }


}
