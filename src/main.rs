use crate::two_sum::two_sum;

mod two_sum;

fn main() {
    test_two_sum();
}

fn test_two_sum() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
}