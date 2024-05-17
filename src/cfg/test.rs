use super::cfg::Cfg;

macro_rules! test {
    ($name:ident, $script:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let cfg = Cfg::new($script);
            assert_eq!(cfg.test($input), $expected);
        }
    };
}

#[allow(unused_macros)]
macro_rules! split_space {
    ($input:expr) => {
        $input.trim().split(" ").collect()
    };
}

macro_rules! split {
    ($input:expr) => {
        $input.trim().split("").filter(|s| s != &"").collect()
    };
}

test!(identifies_true, "S -> a", split!("a"), true);
test!(identifies_false, "S -> a", split!("b"), false);
test!(
    identifies_true_with_epsilon,
    "S -> a | #",
    split!("a"),
    true
);
test!(empty_input_is_false, "S -> a | #", split!(""), false);
test!(
    identifies_true_with_multiple_rules,
    "S -> a \nS -> b",
    split!("b"),
    true
);
test!(
    applies_multiple_rules,
    "S -> A B \nA -> a \nB -> b",
    split!("ab"),
    true
);
test!(
    applies_multiple_rules_in_order,
    "S -> A B \nA -> a \nB -> b",
    split!("ba"),
    false
);
test!(
    applies_multiple_rules_in_order_with_epsilon,
    "S -> A B \nA -> a | # \nB -> b",
    split!("b"),
    true
);

#[test]
fn zeroes_then_ones() {
    let cfg = Cfg::new("S -> 0 S 1 | #");
    assert!(cfg.test(split!("01")));
    assert!(cfg.test(split!("0011")));
    assert!(cfg.test(split!("000111")));
    assert!(!cfg.test(split!("0000111")));
}

#[test]
fn random_binary() {
    let cfg = Cfg::new("S -> 0 S | 1 S | #");
    assert!(!cfg.test(split!("012")));
    assert!(cfg.test(split!("000111")));
    assert!(cfg.test(split!("10100101001")));
    assert!(cfg.test(split!("101001010011")));
}

#[test]
fn any_zeroes_then_ones() {
    let cfg = Cfg::new("S -> Z O\nZ -> 0 Z | #\nO -> 1 O | #");
    assert!(cfg.test(split!("01")));
    assert!(cfg.test(split!("00111111")));
    assert!(cfg.test(split!("000000111")));
    assert!(!cfg.test(split!("00001110")));
}
