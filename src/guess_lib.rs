pub mod guess_number {

    use colored::Colorize;
    use std::cmp::Ordering;
    use std::io::{self, Write};
    use std::time::Duration;
    use std::{thread, time};

    /**
     * Display a welcome message - include the range
     */

    pub fn intro(min: u32, max: u32, secret_number: u32) {
        println!(
            "{}",
            "╔═════════════════════════════════════════════╗"
                .white()
                .on_blue()
        );
        println!(
            "{}",
            "║ Welcome to the amazing number guessing game ║"
                .white()
                .on_blue()
        );
        println!(
            "{}",
            "╚═════════════════════════════════════════════╝"
                .white()
                .on_blue()
        );
        println!("");
        // Fake loading pause duration
        const SHORT_BREAK: Duration = time::Duration::from_millis(150);

        // Display the intro text
        print!("Fake loading");
        for _n in min..secret_number {
            // Display a "." every iteration
            print!(".");
            io::stdout().flush().unwrap();
            // Flush the standout cache to make the content display
            thread::sleep(SHORT_BREAK);
        }
        print!("done! ({secret_number})");
        println!("");
        println!("");
        println!("");
        println!("Guess the number between {min} and {max}!");
        println!("");
        println!("");
    }

    /**
     * Display a welcome message - include a debug hint showing the number.
     */

    pub fn input_helper_txt(tries: i32) {
        print!("You are on try {tries}. Please input your next guess: ");
        io::stdout().flush().unwrap();
    }

    // Handle the reading of the guess.

    pub fn read_guess(mut guess: String, secret_number: u32) -> bool {
        // Prompt the user to provide a number
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Nope, not a number.");
                return false;
            }
        };

        // Match the number to an outcome
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small"),
            Ordering::Greater => println!("{guess} is too big"),
            Ordering::Equal => {
                println!("{guess} is just right.");
                println!("Thanks for playing");
                return true;
            }
        }
        // If no match continue
        return false;
    }
}
