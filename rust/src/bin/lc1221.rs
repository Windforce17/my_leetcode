struct Solution;
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ans = 0;
        let mut count = 0;
        if s.len() == 0 {
            return 0;
        }
        for i in s.chars() {
            match i {
                'R' => count += 1,
                'L' => count -= 1,
                _ => (),
            }
            if count == 0 {
                ans += 1;
            }
        }
        ans
    }
}
fn main() {}
