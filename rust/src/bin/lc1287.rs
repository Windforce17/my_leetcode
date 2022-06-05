struct Solution;
impl Solution {
    pub fn find_special_integer1(arr: Vec<i32>) -> i32 {
        let f = arr.len() / 4;
        let (mut temp, mut count) = (arr[0], 0);
        for i in arr {
            if i == temp {
                count += 1;
                if count > f {
                    return temp;
                }
            } else {
                temp = i;
                count = 1;
            }
        }
        temp
    }
}
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let step = n / 4;

        for i in 0..n - step {
            if arr[i] == arr[i + step] {
                return arr[i];
            }
        }

        return -1;
    }
}
//120100000000
fn main() {
    let v = vec![1, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    println!("{}", Solution::find_special_integer(v));
}
