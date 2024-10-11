use rand::Rng;
use std::io;

fn generate_rand_num() -> u32 {
    return rand::thread_rng().gen_range(1..=100);
}

fn main() {
    // what to print before the game beginning
    println!("===== GUESSING GAME =====");
    println!("Please input your guest.");

    // generate the secret number
    let secret_num: u32 = generate_rand_num();

    // loop until the player find the right secret number
    loop {
        // ask for the the player guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line ⚠️");

        // parse the player guess (String --> u32)
        let guess: u32 = guess.trim().parse().expect("Please type a number! ⚠️");

        // make the macth between the secret number and the player guess
        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("Too small! ❌"),
            std::cmp::Ordering::Greater => println!("Too big! ❌"),
            std::cmp::Ordering::Equal => {
                println!("You win! ✅🎉");
                break;
            }
        }
    }
}
