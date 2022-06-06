struct Solution;
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut count: i32 = 0;
        s.chars().into_iter().for_each(|x| {
            if x == letter {
                count += 1;
            }
        });
        (count as f32 * 100.0 / s.len() as f32) as i32
    }
}
fn main() {
    println!("{}", Solution::percentage_letter("vmvvvvvzrvvpvdvvvvyfvdvvvvpkvvbvvkvvfkvvvkvbvvnvvomvzvvvdvvvkvvvvvvvvvlvcvilaqvvhoevvlmvhvkvtgwfvvzy".into(), 'v'));
}
