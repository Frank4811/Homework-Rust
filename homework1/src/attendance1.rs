/*Write a program
Ask user about his or her car
Create a struct car
Populate from user input
Save struct inside of user_info.txt
Read user_info.txt and print the content on the screen
Submit a screenshot and link to github*/

use std::io::{self, Write, BufReader, BufRead}; // Import for input/output handling, including file reading/writing
use std::fs::{File, OpenOptions}; // Import for file handling and OpenOptions

struct Car {
    name: String,
    year: u32,
}

fn main() {
    let car = get_car_info(); // Get car info from user input

    write_car_info(&car); // Write car info to a file

    read_car_info(); // Read car info from the file and print it to the screen
}

// Function to get car information from the user
fn get_car_info() -> Car {
    let mut name = String::new(); // Variable to store the car's name
    let mut year = String::new(); // Variable to store the car's year

    // Ask the user for the car's name
    print!("Enter the name of your car: ");
    io::stdout().flush().unwrap();  // Ensure the prompt is printed before input
    io::stdin().read_line(&mut name).unwrap(); // Read user input into the `name` String
    let name = name.trim().to_string(); // Remove any leading/trailing spaces and convert input to a String

    // Ask the user for the car's release year
    print!("Enter the year your car was released: ");
    io::stdout().flush().unwrap();  // Ensure the prompt is printed before input
    io::stdin().read_line(&mut year).unwrap();
    let year: u32 = year.trim().parse().expect("Please enter a valid year"); // Convert the input to u32

    // Return a Car struct with the collected data
    Car { name, year }
}

// Function to write car information to a file
fn write_car_info(car: &Car) {
    // Open or create the file for writing, and append new data
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("user_info.txt")
        .expect("Unable to open file");

    // Write the car's name and year to the file in a consistent format
    writeln!(file, "Car Name: {}", car.name).unwrap();
    writeln!(file, "Car Year: {}", car.year).unwrap();
}

// Function to read car information from a file and print it to the console
fn read_car_info() {
    // Open the file for reading
    let file = File::open("user_info.txt").expect("Could not open file");

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Print a header for the output
    println!("\nYOUR CAR INFORMATION:");

    // Loop through each line in the file and print it
    for line in reader.lines() {
        println!("{}", line.unwrap()); // Unwrap each line to get the content
    }
}
