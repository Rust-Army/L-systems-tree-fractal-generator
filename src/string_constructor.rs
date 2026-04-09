use std::collections::HashMap;

/*
    Constructs a final string by running the user defined init_string through the grammar_map for \
    a number of iterations
    Returns: String
*/
pub fn construct_string(init_string: &String, grammar_map: &HashMap<char, String>, iterations: i32) -> String {
    let mut tmp = init_string.to_string();

    for _ in 0..iterations {
        let mut constructed_str = String::new();

        for c in tmp.chars() {
            if let Some(rule) = grammar_map.get(&c) {
                constructed_str.push_str(rule);
            } else {
                constructed_str.push(c);
            }
        }

        tmp = constructed_str;
    }

    tmp
}