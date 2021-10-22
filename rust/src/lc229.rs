
use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec::Vec;
//unfinish
pub struct Solution();
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        #[derive(PartialEq, Eq, PartialOrd)]
        struct Pair{
            data:i32,
            count:i32
        }
        impl Ord for Pair{
            fn cmp(&self, other: &Self)->Ordering {
                self.count.cmp(&other.count)
            }
        }
        let mut position_Map:HashMap<i32,Pair> = HashMap::new();
        for i in &nums{
            let p= position_Map.entry(*i).or_insert(Pair{ data:*i, count: 0});
            p.count+=1;
        }
        let mut p_vec=Vec::new() as Vec<Pair>;
        for(_,v)in position_Map{
            p_vec.push(v);
        }
        //逆序排序
        p_vec.sort();
        let mut result:Vec<i32>=Vec::new();
        for i in 0..(&nums.len()/3){
            result.push(p_vec[i].data)
        }
        
        result
    }
}