use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i64, worker_times: Vec<i64>) -> i64 {
        let mut pq: BinaryHeap<Reverse<(i64, i64, i64)>> = BinaryHeap::new();

        for &t in &worker_times {
            pq.push(Reverse((t, t, t)));
        }

        let mut ans = 0;
        for _ in 0..mountain_height {
            if let Some(Reverse((nxt, delta, base))) = pq.pop() {
                ans = nxt;
                let new_nxt = nxt + delta + base;
                let new_delta = delta + base;
                pq.push(Reverse((new_nxt, new_delta, base)));
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    let mountain_height = nums[0];
    let worker_times_size = nums[1] as usize;
    let worker_times = nums[2..2 + worker_times_size].to_vec();
    let ans = Solution::min_number_of_seconds(mountain_height, worker_times);
    println!("{}", ans);
}