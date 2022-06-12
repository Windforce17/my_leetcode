struct Solution;
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absence_count = 0;
        let mut late_count = 0;
        for i in s.chars() {
            match i {
                'A' => {
                    late_count = 0;
                    absence_count += 1;
                    if absence_count == 2 {
                        return false;
                    }
                }
                'L' => {
                    late_count += 1;
                    if late_count == 3 {
                        return false;
                    }
                }
                'P' => late_count = 0,
                _ => {}
            }
        }
        true
    }
}
fn main() {
    println!("{}", Solution::check_record("PPALLL".to_string()));
}
