use crate::add_two_numbers::{add_two_numbers, ListNode, to_int};
use crate::longest_substring_no_repeating_chars::length_of_longest_substring;
use crate::two_sum::two_sum;

mod two_sum;
mod longest_substring_no_repeating_chars;
mod add_two_numbers;

fn main() {
    println!("{:?}", test_add_two_number());
}

fn test_two_sum() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
}

fn test_longest_substring_no_repeating_chars() {
    println!("{}", length_of_longest_substring("aab".to_string()));
    println!("{}", length_of_longest_substring("abcabcbb".to_string()));
    println!("{}", length_of_longest_substring("bbbbb".to_string()));
    println!("{}", length_of_longest_substring("pwwkew".to_string()));
}

fn test_add_two_number() -> i32 {
    let l1 = ListNode {
        val: 3,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    };
    let l = Some(Box::new(l1));
    let l2 = ListNode {
        val: 4,
        next: Some(Box::new(ListNode { val: 1, next: None })),
    };
    let ll = Some(Box::new(l2));
    let res = add_two_numbers(l, ll);
    to_int(res)
}