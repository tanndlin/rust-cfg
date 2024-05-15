mod cfg;

use crate::cfg::CFG;

fn main() {
    let cfg = CFG::new();

    // Read a string from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    // Test the string against the CFG
    println!("Testing string: {}", input);
    // println!("Result: {}", cfg.test(input));
}
