//https://leetcode.cn/problems/ways-to-make-a-fair-array/
struct Solution;
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut right_sum1 = 0;
        let mut right_sum2 = 0;
        let mut find_plans = 0;
        let mut start_from_1;
        for (i, v) in nums.iter().enumerate() {
            if i % 2 == 0 {
                sum2 += v;
            } else {
                sum1 += v;
            }
        }

        if nums.len() % 2 == 0 {
            start_from_1 = true;
        } else {
            start_from_1 = false;
        }

        for v in nums.into_iter().rev() {
            if start_from_1 {
                right_sum1 += v
            } else {
                right_sum2 += v
            }
        }
        find_plans
    }
}

fn main() {
    Solution::ways_to_make_fair(vec![1, 2, 3, 4]);
}
