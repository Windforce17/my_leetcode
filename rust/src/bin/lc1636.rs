struct Solution;
impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hashmap = HashMap::new();
        nums.iter()
            .for_each(|i| *hashmap.entry(i.clone()).or_insert(0) += 1);
        // if key is equal then cmp with -k
        nums.sort_by_key(|k| (hashmap[k], -k));
        nums
    }
}
fn main() {
    println!("{:?}", Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]));
}
