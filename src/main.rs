use guessing_game::guess_lib::guess_number;


/**
* Guess the number!
*/


fn main() {

    use rand::Rng;

    // Set hard coded range limits
    const MIN: u32 = 10;
    const MAX: u32 = 20;

    // how many ties have we tried?
    let mut tries: i32 = 1;
    
    let secret_number: u32 = rand::thread_rng().gen_range(MIN..=MAX);

    // Send control character 27 to clear the screen
    print!("{}[2J", 27 as char);
    guess_number::intro(MIN, MAX, secret_number);
    

    loop {
        let next_guess: String = String::new();
        
        guess_number::input_helper_txt(tries);
        if guess_number::read_guess(next_guess, secret_number) {
            break;
        } else {
            tries = tries + 1;
        }               
    }
}
