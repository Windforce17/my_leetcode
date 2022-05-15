struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut ov = false;
        for e in digits.iter_mut().rev() {
            *e += 1;
            match e {
                10 => {
                    ov = true;
                    *e = 0;
                }
                _ => ov = false,
            }
        }
        if ov {
            digits.insert(0, 1);
        }
        digits
    }
}

fn main() {
    let a = vec![9, 9, 9];
    println!("{:?}", Solution::plus_one(a));
}
