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
