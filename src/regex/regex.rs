use super::parser::*;
use super::patterns::*;

pub struct Regex {
    pub patterns: Vec<Box<dyn TestablePattern>>,
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
mod test {
    use super::*;

    match_pattern!(matches_single_char, "1", "1", true);
    match_pattern!(does_not_match_single_char, "1", "0", false);
    match_pattern!(matches_multiple_chars, "123", "123", true);
    match_pattern!(matches_range_of_chars2, "1{2,4}", "11", true);
    match_pattern!(matches_range_of_chars3, "1{2,4}", "111", true);
    match_pattern!(matches_range_of_chars4, "1{2,4}", "111", true);
    match_pattern!(matches_range_of_chars5, "1{2,4}", "11111", true);
    match_pattern!(matches_slice, "1{2,2}", "3113", true);
    match_pattern!(matches_any_amount1, "1*", "01", true);
    match_pattern!(matches_any_amount2, "1*", "0", true);
    match_pattern!(matches_any_amount3, "01*2", "011112", true);
    match_pattern!(matches_atleast_one1, "0+", "0", true);
    match_pattern!(matches_atleast_one2, "0+", "00", true);
    match_pattern!(matches_optional1, "10?1", "101", true);
    match_pattern!(matches_optional2, "10?1", "11", true);
}
