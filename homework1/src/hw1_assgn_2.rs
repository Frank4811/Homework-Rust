//  Check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integers
    let numbers: [i32; 10] = [12, 15, 7, 30, 9, 20, 25, 18, 5, 10];

    // Iterate through the array and analyze each number
    for &num in numbers.iter() {
        // Check if the number is even or odd
        if is_even(num) {
            print!("{num} is even. ");
        } else {
            print!("{num} is odd. ");
        }

        // Check for Fizz, Buzz, or FizzBuzz
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!();
        }
    }

    // Find the sum of all numbers using a while loop
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("The sum of all numbers is: {sum}");

    // Find the largest number in the array using a loop
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number in the array is: {largest}");
}
