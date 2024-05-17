use super::patterns::*;
use super::regex::*;

pub fn parse_regex(pattern: &str) -> Regex {
    let mut patterns: Vec<Box<dyn TestablePattern>> = Vec::new();
    let chars: Vec<_> = pattern.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let token_selector = parse_token_selector(&chars, &mut i);
        patterns.push(parse_amount_specifier(&chars, &mut i, token_selector));
    }

    Regex { patterns }
}

fn parse_token_selector(chars: &[char], i: &mut usize) -> Box<dyn TokenSelector> {
    if chars[*i] == '[' {
        consume_char(chars[*i], '[', i);
        let mut tokens = Vec::new();
        while chars[*i] != ']' {
            tokens.push(chars[*i]);
            *i += 1;
        }

        consume_char(chars[*i], ']', i);
        Box::new(MultiCharSelector { tokens })
    } else {
        let ret = Box::new(SingleCharSelector { token: chars[*i] });
        *i += 1;
        ret
    }
}

fn parse_number_specifier(chars: &[char], i: &mut usize) -> (usize, usize) {
    consume_char(chars[*i], '{', i);
    let mut min_str = String::new();
    while chars[*i].is_ascii_digit() {
        min_str.push(chars[*i]);
        *i += 1;
    }

    consume_char(chars[*i], ',', i);
    let mut max_str = String::new();
    while chars[*i].is_ascii_digit() {
        max_str.push(chars[*i]);
        *i += 1;
    }

    consume_char(chars[*i], '}', i);
    (
        min_str.parse::<usize>().unwrap(),
        max_str.parse::<usize>().unwrap(),
    )
}

fn parse_amount_specifier(
    chars: &[char],
    i: &mut usize,
    token_selector: Box<dyn TokenSelector>,
) -> Box<dyn TestablePattern> {
    let next_char = chars.get(*i).unwrap_or(&'\0');

    match next_char {
        '*' => {
            *i += 1;
            Box::new(AnyAmountPattern { token_selector })
        }
        '+' => {
            *i += 1;
            Box::new(AtLeastOnePattern { token_selector })
        }
        '?' => {
            *i += 1;
            Box::new(OptionalPattern { token_selector })
        }
        '{' => {
            let (min, max) = parse_number_specifier(chars, i);
            Box::new(BoundedAmountPattern {
                token_selector,
                min_amount: min,
                max_amount: max,
            })
        }

        _ => Box::new(ExactAmountPattern {
            token_selector,
            amount: 1,
        }),
    }
}

fn consume_char(input: char, expected: char, index: &mut usize) {
    if input != expected {
        panic!("Expected {}. Got {}", expected, input);
    }

    *index += 1;
}
