mod lc189;
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    lc189::Solution::rotate(&mut nums, 3);
    println!("{:?}", nums);
}
