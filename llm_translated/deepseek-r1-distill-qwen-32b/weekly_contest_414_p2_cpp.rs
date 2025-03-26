use std::io;

struct Solution;

impl Solution {
    fn max_possible_score(&self, mut start: Vec<i32>, d: i32) -> i32 {
        start.sort();
        let n = start.len();
        if n == 0 {
            return 0;
        }

        let first = start[0] as i64;
        let last = start[n - 1] as i64;
        let numerator = last + d as i64 - first;
        let denominator = (n - 1) as i64;
        let right_initial = (numerator / denominator) as i32 + 1;

        let check = {
            let sorted_start = start.clone();
            move |score: i32| -> bool {
                let mut x = i64::MIN;
                for &s in &sorted_start {
                    let s_i64 = s as i64;
                    let score_i64 = score as i64;
                    x = x.max(x + score_i64, s_i64);
                    if x > s_i64 + d as i64 {
                        return false;
                    }
                }
                true
            }
        };

        let mut left = 0;
        let mut right = right_initial;
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
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let start_size = tokens[0].parse::<i32>().unwrap();
    let d = tokens[1].parse::<i32>().unwrap();
    let start: Vec<i32> = tokens[2..2 + start_size as usize]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution;
    println!("{}", solution.max_possible_score(start, d));
}