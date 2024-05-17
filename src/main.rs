mod cfg;

#[cfg(test)]
mod test;

use crate::cfg::CFG;

fn main() {
    let cfg_txt = std::fs::read_to_string("cfg.txt").unwrap();

    let cfg = CFG::new(&cfg_txt);

    // Read a string from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    // Test the string against the CFG
    println!("Testing string: {}", input);
    println!("Result: {}", cfg.test(input));

    let sample = cfg.generate_sample_langauge(5);
    println!("Sample: {:?}", sample);
}
