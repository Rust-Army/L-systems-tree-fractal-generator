/*
    F -- draws straight line
    B -- draws right curve line
    C -- draws left curve line
    + -- turns right
    - -- turns left
*/
use std::io;
use std::collections::HashMap;

/*
    User creates a grammar by mapping a character key to a string.
    Returns: HashMap<char, String>
*/
pub fn get_system_params() -> HashMap<char, String> {
    let mut grammar_map: HashMap<char, String> = HashMap::new();

    let nts = ['F', 'B', 'C'];

    println!("\nYou will now create your grammar!");

    for nt in nts {
        println!("\nWhat do you want the non-terminal (NT) {} to map to?", nt);
        let mut tmp = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("Failed to read line");

        grammar_map.insert(nt, tmp.trim().to_string());
    }

    grammar_map
}

/*
    User sets the number of iterations to run the initial string through.
    Returns: i32
*/
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

/*
    User sets the initial string.
    Returns: String
*/
pub fn get_init_string() -> String {
    let mut input = String::new();

    println!("What do you want your initial string to be?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let init_string = input.trim();

    init_string.to_string()
}