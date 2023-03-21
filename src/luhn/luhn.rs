// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut arr : Vec<u32> = Vec::new();
    for num in cc_number.chars().filter(|c| !c.is_whitespace()){
        let n: Option<u32> = num.to_digit(10);
        if let Some(v) = n{
            arr.push(v);
        }
    }
    if arr.len() < 2 { return false; }
    let mut sum = 0;
    for (i,num) in arr.iter().rev().enumerate() {
        let doubled = num*2;
        if i&1 == 1 && doubled != 18 {
            sum += doubled % 9;
        }else{
            sum += num;
        }
    }
    sum%10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    let val = luhn(" 0 0 ");
    assert!(val);
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}