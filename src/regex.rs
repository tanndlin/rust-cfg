pub struct Regex {
    patterns: Vec<Box<dyn TestablePattern>>,
}

trait TestablePattern {
    fn test(&self, input: &str) -> (bool, usize);
}

pub struct ExactAmountPattern {
    tokens: Vec<char>,
    amount: usize,
}

impl TestablePattern for ExactAmountPattern {
    fn test(&self, input: &str) -> (bool, usize) {
        let mut index = 0;
        while index < self.amount {
            let c = input.chars().nth(index).unwrap();

            if self.tokens.iter().any(|t| t == &c) {
                index += 1;
            } else {
                return (false, 0);
            }
        }

        (index == self.amount, self.amount)
    }
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

    for c in pattern.chars() {
        regex.patterns.push(Box::new(ExactAmountPattern {
            tokens: vec![c],
            amount: 1,
        }))
    }

    regex
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
    match_pattern!(matches_multiple_chars, "12", "12", true);
    match_pattern!(matches_multiple_chars2, "123", "123", true);
}
