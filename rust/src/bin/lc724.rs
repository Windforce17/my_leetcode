struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = nums.iter().sum::<i32>();
        let mut left_sum: i32 = 0;
        for i in 0..nums.len() {
            sum -= nums[i];
            if sum == left_sum {
                return i as i32;
            }
            left_sum += nums[i];
        }
        -1
    }
}
fn main() {
    println!("{}", Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
}
