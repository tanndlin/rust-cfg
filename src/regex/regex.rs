use super::patterns::*;

pub struct Regex {
    patterns: Vec<Box<dyn TestablePattern>>,
}

pub struct Match {
    captured: String,
}

impl Regex {
    pub fn new(pattern: &str) -> Regex {
        parse_regex(pattern)
    }

    pub fn test(&self, string: &str) -> bool {
        for s in 0..string.len() {
            if self.test_substr(&string[s..]) {
                return true;
            }
        }

        false
    }

    fn test_substr(&self, string: &str) -> bool {
        let mut index = 0;
        for p in self.patterns.iter() {
            let (matches, offset) = p.test(&string[index..]);
            if !matches {
                return false;
            }

            index += offset;
        }

        true
    }
}

fn parse_regex(pattern: &str) -> Regex {
    let mut regex = Regex { patterns: vec![] };
    let chars: Vec<_> = pattern.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if i == chars.len() - 1 || chars[i + 1] != '{' {
            regex.patterns.push(Box::new(ExactAmountPattern {
                tokens: vec![c],
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

        regex.patterns.push(Box::new(BoundedAmountPattern {
            tokens: vec![token],
            min_amount: min_str.parse::<usize>().unwrap(),
            max_amount: max_str.parse::<usize>().unwrap(),
        }))
    }

    regex
}

fn consume_char(input: char, expected: char, index: &mut usize) {
    if input != expected {
        panic!("Expected {}. Got {}", expected, input);
    }

    *index += 1;
}

macro_rules! match_pattern {
    ($name:ident, $pattern:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let regex = Regex::new($pattern);
            let input = $input;

            assert_eq!(regex.test(input), $expected);
        }
    };
}

#[cfg(test)]
mod regex_tests {
    use super::*;

    match_pattern!(matches_single_char, "1", "1", true);
    match_pattern!(does_not_match_single_char, "1", "0", false);
    match_pattern!(matches_multiple_chars, "123", "123", true);
    match_pattern!(matches_range_of_chars2, "1{2,4}", "11", true);
    match_pattern!(matches_range_of_chars3, "1{2,4}", "111", true);
    match_pattern!(matches_range_of_chars4, "1{2,4}", "111", true);
    match_pattern!(matches_range_of_chars5, "1{2,4}", "11111", true);
    match_pattern!(matches_slice, "1{2,2}", "3113", true);
}
