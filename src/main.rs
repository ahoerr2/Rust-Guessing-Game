use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {


    let start_range: u32 = 1;
    let end_range: u32 = 100;
    let mut low_bound: u32 = start_range;
    let mut high_bound: u32 = end_range;
    let mut num_attempts: u32 = 0;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(start_range..=end_range);

    // println!("The secret number is: {secret_number}");

    loop {

        println!("Please Input Your Guess [{0}-{1}] (Best guess: {2})",
            start_range,
            end_range,
            (high_bound + low_bound) / 2
        );

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        num_attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                low_bound = guess;

            },
            Ordering::Greater => {
                println!("Too big!");
                high_bound = guess;
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }     
        
    }

    println!("Number of attempts: {num_attempts}");

}



