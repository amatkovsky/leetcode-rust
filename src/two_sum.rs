use std::collections::HashMap;

// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dict = HashMap::new();
    for i in 0..=nums.len() - 1 {
        dict.insert(nums[i], i);
    }


    for i in 0..nums.len() - 1 {
        let x = target - nums[i];
        if dict.contains_key(&x) && dict[&x] != i {
            return vec![i as i32, dict[&x] as i32]
        }
    }

    panic!("No pair to sum to target found!")
}