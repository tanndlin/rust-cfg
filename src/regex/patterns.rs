pub trait TokenSelector {
    fn contains(&self, c: char) -> bool;
}

pub struct SingleCharSelector {
    pub token: char,
}

impl TokenSelector for SingleCharSelector {
    fn contains(&self, c: char) -> bool {
        c == self.token
    }
}

pub struct MultiCharSelector {
    pub tokens: Vec<char>,
}

impl TokenSelector for MultiCharSelector {
    fn contains(&self, c: char) -> bool {
        self.tokens.contains(&c)
    }
}

pub trait TestablePattern {
    fn test(&self, input: &str) -> (bool, usize);
}

pub struct ExactAmountPattern {
    pub token_selector: Box<dyn TokenSelector>,
    pub amount: usize,
}

impl TestablePattern for ExactAmountPattern {
    fn test(&self, input: &str) -> (bool, usize) {
        let mut index = 0;
        while index < self.amount {
            let c = input.chars().nth(index).unwrap();

            if self.token_selector.contains(c) {
                index += 1;
            } else {
                return (false, 0);
            }
        }

        (index == self.amount, 1)
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