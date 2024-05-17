use regex::Regex;

mod cfg;
mod production;
mod regex;

#[cfg(test)]
mod test;

fn main() {
    let regex = Regex::new("1{2,4}");
    let input = "11111";

    println!("Matching {}: {}", input, regex.test(input));
}
