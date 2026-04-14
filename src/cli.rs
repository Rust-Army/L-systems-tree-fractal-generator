/*
    Ideas for what characters can mean:
    F -- draws straight line
    B -- draws right curve line
    C -- draws left curve line
    + -- turns right
    - -- turns left
*/
use std::io;

pub fn get_number_iterations() -> i32 {
    let mut tmp = String::new();

    println!("\nHow many iterations would you like?");

    io::stdin()
        .read_line(&mut tmp)
        .expect("Failed to read line");

    let iterations: i32 = tmp
        .trim()
        .parse()
        .expect("Please enter a valid number");

    iterations
}

pub fn get_init_string() -> String {
    let mut input = String::new();

    println!("What do you want your initial string to be?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
