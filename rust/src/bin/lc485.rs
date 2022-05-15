struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        for i in nums {
            if i == 1 {
                count += 1;
            } else {
                if count >= max {
                    max = count
                }
                count = 0;
            }
        }
        if count >= max {
            return count;
        }
        max
    }
}
fn main() {
    println!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
    )
}
