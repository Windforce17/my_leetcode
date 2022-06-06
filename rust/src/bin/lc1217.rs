struct Solution;
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut cost = std::i32::MAX;
        use std::collections::HashMap;
        let mut position_map = HashMap::new();
        for i in position {
            match position_map.get_mut(&i) {
                Some(t) => *t += 1,
                None => {
                    position_map.insert(i, 1);
                }
            }
        }
        for (k, _) in &position_map {
            let mut tmp_cost = 0;
            for (k1, v1) in &position_map {
                tmp_cost += ((k1 - k).abs() % 2) * v1;
            }
            if tmp_cost < cost {
                cost = tmp_cost;
            }
        }
        cost
    }
    pub fn min_cost_to_move_chips1(position: Vec<i32>) -> i32 {
        let odds = (position.iter().filter(|x| *x & 1 == 1).count());
        (position.len() - odds).min(odds) as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_cost_to_move_chips1(vec![6, 4, 7, 8, 2, 10, 2, 7, 9, 7])
    );
}
