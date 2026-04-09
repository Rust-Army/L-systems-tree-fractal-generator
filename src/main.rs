mod cli;
mod string_constructor;

use cli::get_system_params;
use cli::get_number_iterations;
use cli::get_init_string;
use string_constructor::construct_string;
use std::collections::HashMap;

fn main() {
    let init_string: String = get_init_string();
    let grammar_map: HashMap<char, String> = get_system_params();
    let iterations: i32 = get_number_iterations();
    let constructed_str: String = construct_string(&init_string, &grammar_map, iterations);

    println!("\n");
    println!("{:?}", init_string);
    println!("{:?}", grammar_map);
    println!("{:?}", iterations);
    println!("{:?}", constructed_str);
}
