use crate::cfg::CFG;

macro_rules! test {
    ($name:ident, $script:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let cfg = CFG::new($script);
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
    let cfg = CFG::new("S -> 0 S 1 | #");
    assert_eq!(cfg.test(split!("01")), true);
    assert_eq!(cfg.test(split!("0011")), true);
    assert_eq!(cfg.test(split!("000111")), true);
    assert_eq!(cfg.test(split!("0000111")), false);
}

#[test]
fn random_binary() {
    let cfg = CFG::new("S -> 0 S | 1 S | #");
    assert_eq!(cfg.test(split!("012")), false);
    assert_eq!(cfg.test(split!("000111")), true);
    assert_eq!(cfg.test(split!("10100101001")), true);
    assert_eq!(cfg.test(split!("101001010011")), true);
}

#[test]
fn any_zeroes_then_ones() {
    let cfg = CFG::new("S -> Z O\nZ -> 0 Z | #\nO -> 1 O | #");
    assert_eq!(cfg.test(split!("01")), true);
    assert_eq!(cfg.test(split!("00111111")), true);
    assert_eq!(cfg.test(split!("000000111")), true);
    assert_eq!(cfg.test(split!("00001110")), false);
}
