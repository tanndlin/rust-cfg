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

test!(identifies_true, "S -> a", "a", true);
test!(identifies_false, "S -> a", "b", false);
test!(identifies_true_with_epsilon, "S -> a | #", "a", true);
test!(empty_input_is_false, "S -> a | #", "", false);
test!(
    identifies_true_with_multiple_rules,
    "S -> a \nS -> b",
    "b",
    true
);
test!(
    applies_multiple_rules,
    "S -> A B \nA -> a \nB -> b",
    "ab",
    true
);
test!(
    applies_multiple_rules_in_order,
    "S -> A B \nA -> a \nB -> b",
    "ba",
    false
);
test!(
    applies_multiple_rules_in_order_with_epsilon,
    "S -> A B \nA -> a | # \nB -> b",
    "b",
    true
);

#[test]
fn zeroes_then_ones() {
    let cfg = CFG::new("S -> 0 S 1 | #");
    assert_eq!(cfg.test("01"), true);
    assert_eq!(cfg.test("0011"), true);
    assert_eq!(cfg.test("000111"), true);
    assert_eq!(cfg.test("0000111"), false);
}

#[test]
fn random_binary() {
    let cfg = CFG::new("S -> 0 S | 1 S | #");
    assert_eq!(cfg.test("10100101001"), true);
    assert_eq!(cfg.test("101001010011"), true);
    assert_eq!(cfg.test("000111"), true);
    assert_eq!(cfg.test("012"), false);
}

#[test]
fn any_zeroes_then_ones() {
    let cfg = CFG::new("S -> Z O\nZ -> 0 Z | #\nO -> 1 O | #");
    assert_eq!(cfg.test("01"), true);
    assert_eq!(cfg.test("00111111"), true);
    assert_eq!(cfg.test("000000111"), true);
    assert_eq!(cfg.test("00001110"), false);
}
