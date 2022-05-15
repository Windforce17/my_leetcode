pub struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut k = k;
        while k != 0 {
            let t = nums.pop().unwrap();
            nums.insert(0, t);
            k -= 1;
        }
    }
}

// impl Solution {
//     pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//         let rem = k as usize % nums.len();
//         nums.reverse();
//         nums[0..rem].reverse();
//         nums[rem..].reverse();
//     }
// }
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    println!("{:?}", nums);
}
