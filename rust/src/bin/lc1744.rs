// https://leetcode-cn.com/problems/can-you-eat-your-favorite-candy-on-your-favorite-day/
pub struct Solution{}

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut answer:Vec<bool> = Vec::new();

        for query in queries {
            let (candy_type,day,cap)=(query[0],query[1],query[2]);
            let mut sum_of_eat=0;
            let mut i:usize=0;
            while i< (candy_type) as usize&&i<candies_count.len() {
                sum_of_eat+=candies_count[i];
                i+=1;
            }
            let min_day=sum_of_eat/cap;
            let max_day=sum_of_eat+candies_count[candy_type as usize];
            answer.push(day>=min_day&&day<=max_day-1);
            
            
        }
        answer
    }
}