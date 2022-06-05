struct Solution;
impl Solution {
    pub fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut position_map = HashMap::new();
        intervals.iter().enumerate().for_each(|(i, e)| {
            position_map.insert(e.clone(), i);
        });
        intervals.sort_by_key(|x| x[0]);
        let mut ans: Vec<i32> = vec![0; intervals.len()];
        intervals.iter().for_each(|x| {
            let p = intervals
                .binary_search_by_key(&x[1], |k| k[0])
                .unwrap_or_else(|k| k);
            if p >= intervals.len() || intervals[p][0] < x[1] {
                ans[position_map[x]] = -1;
            } else {
                ans[position_map[x]] = position_map[&intervals[p]] as i32;
            }
        });
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]])
    )
}
