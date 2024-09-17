use rand::Rng; // Import the random number generator

// Function to check the guess against the secret number
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random secret number between 1 and 100
    let secret: i32 = rng.gen_range(1..=100);
    
    // Variable to keep track of the number of guesses
    let mut num_guesses: i32 = 0;

    // Start the guessing loop
    loop {
        // Generate a random guess between 1 and 100
        let guess: i32 = rng.gen_range(1..=100);
        println!("Your guess: {guess}");

        // Increment guess count
        num_guesses += 1;

        // Check the guess using the check_guess function
        let result = check_guess(guess, secret);

        // If-else expression to handle the result of the guess
        if result == 0 {
            println!("Congratulations! You guessed the correct number.");
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("Your guess is too high.");
        } else {
            println!("Your guess is too low.");
        }
    }

    println!("It took you {num_guesses} guesses to find the correct number.");
}
