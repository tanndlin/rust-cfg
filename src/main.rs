use regex::Regex;

mod cfg;
mod production;
mod regex;

#[cfg(test)]
mod test;

fn main() {
    let regex = Regex::new("123");
    let input = "123";

    println!("Matching {}: {}", input, regex.test(input));
}
