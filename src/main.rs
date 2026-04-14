mod cli;
mod string_constructor;
mod renderer;

use cli::get_number_iterations;
use cli::get_init_string;
use string_constructor::construct_string;
use std::collections::HashMap;

fn main() {
    let mut grammar_map: HashMap<char, String> = HashMap::new();
    grammar_map.insert('F', String::from("F[+F][-F]F"));

    println!("grammar: {:?}", grammar_map);

    let init_string: String = get_init_string();
    let iterations: i32 = get_number_iterations();
    let constructed_str: String = construct_string(&init_string, &grammar_map, iterations);

    renderer::render(&constructed_str);
}
