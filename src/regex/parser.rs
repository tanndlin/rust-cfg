use super::patterns::*;
use super::regex::*;

pub fn parse_regex(pattern: &str) -> Regex {
    let mut patterns: Vec<Box<dyn TestablePattern>> = Vec::new();
    let chars: Vec<_> = pattern.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if i == chars.len() - 1 || chars[i + 1] != '{' {
            patterns.push(Box::new(ExactAmountPattern {
                token_selector: Box::new(SingleCharSelector { token: c }),
                amount: 1,
            }));

            i += 1;
            continue;
        }

        let token = chars[i];
        i += 1;

        consume_char(chars[i], '{', &mut i);
        let mut min_str = String::new();
        while chars[i].is_ascii_digit() {
            min_str.push(chars[i]);
            i += 1;
        }

        consume_char(chars[i], ',', &mut i);
        let mut max_str = String::new();
        while chars[i].is_ascii_digit() {
            max_str.push(chars[i]);
            i += 1;
        }
        consume_char(chars[i], '}', &mut i);

        patterns.push(Box::new(BoundedAmountPattern {
            tokens: vec![token],
            min_amount: min_str.parse::<usize>().unwrap(),
            max_amount: max_str.parse::<usize>().unwrap(),
        }))
    }

    Regex { patterns }
}

fn consume_char(input: char, expected: char, index: &mut usize) {
    if input != expected {
        panic!("Expected {}. Got {}", expected, input);
    }

    *index += 1;
}
