struct Solution;
impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut min_id_workder = n + 1;
        let mut last_done_time = 0;
        let mut max_time = -1;
        for i in logs {
            let id = i[0];
            let leave_time = i[1];
            let work_time = leave_time - last_done_time;
            if work_time == max_time {
                if id < min_id_workder {
                    min_id_workder = id;
                }
            } else if work_time > max_time {
                max_time = work_time;
                min_id_workder = id;
            }
            last_done_time = leave_time;
        }
        return min_id_workder;
    }
}

fn main() {
    let n = 10;
    // [[0,3],[2,5],[0,9],[1,15]]
    let logs = vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]];
    let result = Solution::hardest_worker(n, logs);
    println!("result = {}", result);
}
