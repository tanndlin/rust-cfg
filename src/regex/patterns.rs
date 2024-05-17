pub trait TokenSelector {
    fn contains(&self, c: char) -> bool;
}

pub trait TestablePattern {
    fn test(&self, input: &str) -> (bool, usize);
}

pub struct ExactAmountPattern {
    pub tokens: Vec<char>,
    pub amount: usize,
}

impl TestablePattern for ExactAmountPattern {
    fn test(&self, input: &str) -> (bool, usize) {
        let mut index = 0;
        while index < self.amount {
            let c = input.chars().nth(index).unwrap();

            if self.tokens.contains(&c) {
                index += 1;
            } else {
                return (false, 0);
            }
        }

        (index == self.amount, self.amount)
    }
}

pub struct BoundedAmountPattern {
    pub tokens: Vec<char>,
    pub min_amount: usize,
    pub max_amount: usize,
}

impl TestablePattern for BoundedAmountPattern {
    fn test(&self, input: &str) -> (bool, usize) {
        let mut index = 0;
        let mut amount_matched = 0;
        loop {
            if index == input.len() {
                break;
            }

            let c = input.chars().nth(index).unwrap();

            if self.tokens.contains(&c) {
                amount_matched += 1;
                index += 1;
            } else {
                break;
            }
        }

        let matched = self.min_amount <= index && index <= self.max_amount;
        (matched, amount_matched)
    }
}
