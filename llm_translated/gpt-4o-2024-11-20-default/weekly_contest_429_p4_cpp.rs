// Problem: Weekly Contest 429 Problem 4
use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Method to merge intervals given a maximum length restriction
    fn merge(intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut intervals = intervals;
        let mut res = vec![intervals.remove(0)];

        for interval in intervals {
            let curr_start = interval[0];
            let curr_end = interval[1];

            if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len {
                res.last_mut().unwrap()[1] = max(res.last().unwrap()[1], curr_end);
            } else {
                res.push(interval);
            }
        }

        res
    }

    // Method to check if the operation is possible given constraints
    fn is_poss(s: &str, op: i32, mid: i32) -> bool {
        let n = s.len() as i32;
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = vec![];

        while j < n {
            if &s[j as usize..j as usize + 1] == "0" {
                zero += 1;
            } else {
                one += 1;
            }

            while (j - i + 1) > mid {
                if &s[i as usize..i as usize + 1] == "0" {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            if (j - i + 1) == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i, j]);
                }
            }

            j += 1;
        }

        let merged = Self::merge(intervals, mid);
        merged.len() as i32 <= op
    }

    // Helper method to calculate minimum operations needed for alternating characters
    fn get_mini(s: &str, even: char, odd: char) -> i32 {
        let n = s.len();
        let mut op = 0;

        for i in 0..n {
            let expected = if i % 2 == 0 { even } else { odd };
            if s.chars().nth(i).unwrap() != expected {
                op += 1;
            }
        }

        op
    }

    // Main method to calculate the minimum length
    fn min_length(s: String, num_ops: i32) -> i32 {
        let n = s.len() as i32;
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        let mut op = n + 1;
        op = op.min(Self::get_mini(&s, '0', '1'));
        op = op.min(Self::get_mini(&s, '1', '0'));
        if op <= num_ops {
            return 1;
        }

        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = Self::is_poss(&s, num_ops, mid);

            if curr {
                end = mid - 1;
            } else {
                res = mid;
                start = mid + 1;
            }
        }

        res
    }
}

fn main() {
    // Input handling
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s = lines.next().unwrap().unwrap(); // Read the binary string
    let num_ops: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap(); // Read the number of operations

    // Compute the result
    let solution = Solution;
    let result = solution.min_length(s, num_ops);

    // Output the result
    println!("{}", result);
}