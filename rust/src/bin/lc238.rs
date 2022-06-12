struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(nums.len());
        let mut l = 1;
        ans.push(1);
        for i in 1..nums.len() {
            ans.push(ans[i - 1] * nums[i - 1]);
        }
        for i in (0..nums.len()).rev() {
            ans[i] *= l;
            l *= nums[i];
        }
        ans
    }
}

fn main() {
    for i in (0..4).rev() {
        println!("{}", i);
    }
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
    //[24,12,8,6]
}
