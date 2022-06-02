struct Solution;
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let ans: Vec<i32> = Vec::with_capacity(mat.len() * mat[0].len());
        let m = mat.len();
        let n = mat[0].len();
        let (i, j) = (0, 0);
        enum Direction {
            Up,
            Down,
        }
        while i != m && j != n {}
        ans
    }
}

fn main() {
    unimplemented!();
}
