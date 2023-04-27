use crate::longest_substring_no_repeating_chars::length_of_longest_substring;
use crate::two_sum::two_sum;

mod two_sum;
mod longest_substring_no_repeating_chars;

fn main() {
    test_longest_substring_no_repeating_chars()
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