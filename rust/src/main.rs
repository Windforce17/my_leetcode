mod lc1744;
fn main() {
    let a=vec![16,38,8,41,30,31,14,45,3,2,24,23,38,30,31,17,35,4,9,42,28,18,37,18,14,46,11,13,19,3,5,39,24,48,20,29,4,19,36,11,28,49,38,16,23,24,4,22,29,35,45,38,37,40,2,37,8,41,33,8,40,27,13,4,33,5,8,14,19,35,31,8,8];
    let b=vec![vec![43,1054,49]];
    //false

    // let a=vec![5,2,6,4,1];
    // let b=vec![vec![3,1,2],vec![4,10,3],vec![3,10,100],vec![4,100,30],vec![1,3,1]];
    println!("{:?}",lc1744::Solution::can_eat(a, b));
}

}
