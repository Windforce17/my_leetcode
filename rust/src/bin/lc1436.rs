struct Solution;
impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        (nums[0] - 1) * (nums[1] - 1)
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![3, 4, 5, 2]));
}
