use std::io;

struct Solution;

impl Solution {
    pub fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
        start.sort_unstable();
        let check = |score: i32| -> bool {
            let mut x = i64::MIN;
            for &s in start.iter() {
                x = x.max(x + score as i64);
                if x > (s as i64 + d as i64) {
                    return false;
                }
            }
            true
        };

        let first = start[0];
        let last = *start.last().unwrap();
        let n = start.len();
        let denominator = (n - 1) as i32;
        let numerator = (last - first) as i32 + d;
        let mut right = (numerator / denominator) + 1;
        let mut left = 0;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let start_size = iter.next().unwrap();
    let d = iter.next().unwrap();

    let mut start = Vec::with_capacity(start_size as usize);
    for _ in 0..start_size {
        start.push(iter.next().unwrap());
    }

    let solution = Solution;
    let result = solution.max_possible_score(start, d);
    println!("{}", result);
}