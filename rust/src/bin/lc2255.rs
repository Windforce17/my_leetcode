#![feature(pattern)]
use std::str::pattern::Pattern;

struct Solution;
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        use std::collections::HashMap;
        let mut w = HashMap::new();
        let mut ans = 0;
        for i in words {
            if let Some(k) = w.get(&i) {
                w.insert(i, k + 1);
            } else {
                w.insert(i, 1);
            }
        }
        for i in 1..s.len() + 1 {
            if let Some(v) = w.get(&s[0..i]) {
                ans += v;
            }
        }

        ans
    }
    pub fn count_prefixes1(words: Vec<String>, s: String) -> i32 {
        let mut ans = 0;
        words.iter().for_each(|x| {
            if x.is_prefix_of(&s) {
                ans += 1;
            }
        });
        ans
    }
}

fn main() {
    let mut input1 = vec!["a", "b", "c", "ab", "bc", "abc"];
    let input1 = input1.iter_mut().map(|x| x.to_string()).collect();

    println!("{}", Solution::count_prefixes1(input1, "abc".to_string()));
}
