mod cfg;
mod production;

#[cfg(test)]
mod test;

use crate::cfg::Cfg;

fn main() {
    let cfg_txt = std::fs::read_to_string("cfg.txt").unwrap();

    let cfg = Cfg::new(&cfg_txt);

    // Read a string from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.trim().split(' ').collect();

    // Test the string against the CFG
    println!("Testing string: {:?}", input);
    println!("Result: {}", cfg.test(input));

    let sample = cfg.generate_sample_langauge(10);

    println!("Sample language:");
    for s in sample.iter() {
        println!("{}", s);
    }
}
