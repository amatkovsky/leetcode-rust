use std::cmp::max;
use std::collections::HashMap;

// https://leetcode.com/problems/longest-substring-without-repeating-characters/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut right = 0;

    let mut freq_map = HashMap::new();

    let mut len = 0;

    let bytes = s.as_bytes();
    while right <= bytes.len() {
        if right == bytes.len() {
            len = max(len, right - left);
            return len as i32;
        }
        let char = bytes[right];
        let freq = freq_map.entry(char).or_insert(0);
        if *freq == 1 {
            // there is already a symbol in substring
            // move left edge to the right
            len = max(len, right - left);
            freq_map.insert(bytes[left], 0);
            left += 1;
        } else {
            freq_map.insert(char, 1);
            right += 1;
        }
    }

    len as i32
}