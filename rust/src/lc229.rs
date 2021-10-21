
use std::collections::HashMap;
//unfinish
pub struct Solution();
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        struct Pair{
            data:i32,
            count:i32
        }
        let mut positionMap:HashMap<i32,Pair> = HashMap::new();
        for i in nums{
            let p= positionMap.entry(i).or_insert(Pair{ data:i, count: 0});
            p.count+=1;
        }
        vec![11,2]
    }
}