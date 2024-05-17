mod cfg;
mod regex;
use regex::regex::Regex;

fn main() {
    let regex = Regex::new("a{1,2}");
    let input = "aaa";

    println!("{:?}", regex.test(input));
}
