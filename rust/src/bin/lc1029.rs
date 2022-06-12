struct Solution;
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        // only need once for loop.
        let mut a_sum = costs.iter().fold(0, |x, y| x + y[0]);
        let mut a_sub_b: Vec<i32> = costs.iter().map(|x| x[0] - x[1]).collect();
        a_sub_b.sort();
        let n = costs.len() / 2;
        for i in a_sub_b[n..].iter().rev() {
            a_sum -= i;
        }
        a_sum
    }
}
fn main() {
    println!(
        "{}",
        Solution::two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20]
        ])
    );
}
