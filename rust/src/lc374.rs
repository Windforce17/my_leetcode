pub struct Solution();
impl Solution {
    pub unsafe fn guessNumber(n: i32) -> i32 {
        Self::guess_number(n, n)
        
    }
    fn guess_number(n:i32,max:i32)->i32{
        match Self::guess(n) {
            0 => n,
            -1 => Solution::guess_number(n / 2,n),
            1 => Solution::guess_number(n+(max-n)/2,max),
            _=>unreachable!()
        }
    }
    fn guess(n: i32) -> i32 {
        const NUM: i32 = 50;
        match n {
            _k if n == NUM => 0,
            _k if n > NUM => -1,
            _k if n < NUM => 1,
            _ => unreachable!()
        }
    }
}
