struct Solution();

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_sum: Vec<u32> = Vec::with_capacity(candies_count.len());
        prefix_sum.push(candies_count[0] as u32);
        for i in 1..candies_count.len() {
            prefix_sum.push(prefix_sum[i - 1] + candies_count[i] as u32);
        }
        queries
            .iter()
            .map(move|query| {
                let (candy_type, day, cap) = (query[0] , query[1]  as u32, query[2] );
                let min_day = if 0 == candy_type {
                    0
                } else {
                    prefix_sum[candy_type as usize - 1] / cap as u32
                };
                let max_day = prefix_sum[candy_type as usize] - 1;
                day >= min_day && day <= max_day
            })
            .collect()
    }
}