struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (small_str, big_str) = if str1.len() >= str2.len() {
            (str2, str1)
        } else {
            (str1, str2)
        };
        let (mut small_success, mut big_success) = (false, false);
        let mut answer_vec = std::collections::BinaryHeap::new();
        for i in 1..small_str.len() + 1 {
            for s in big_str.as_bytes().chunks(i) {
                if s != &small_str.as_bytes()[0..i] {
                    big_success = false;
                    break;
                } else {
                    big_success = true;
                }
            }
            if !big_success {
                continue;
            }
            for s in small_str.as_bytes().chunks(i) {
                if s != &small_str.as_bytes()[0..i] {
                    small_success = false;
                    break;
                } else {
                    small_success = true;
                }
            }
            if small_success && big_success {
                answer_vec.push(&small_str[0..i]);
            }
        }
        return answer_vec.peek().map_or("".to_string(), |x| x.to_string());
    }
}
fn main() {
    println!(
        "{}",
        Solution::gcd_of_strings("ABABABAB".into(), "ABAB".into())
    );
}
