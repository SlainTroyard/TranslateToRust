use std::io;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let n = intervals.len();

        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(intervals[0].clone());

        for i in 1..n {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];

            if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len {
                res.last_mut().unwrap()[1] = max(res.last().unwrap()[1], curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    pub fn is_poss(&self, s: &str, op: i32, mid: i32) -> bool {
        let n = s.len() as i32;
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals: Vec<Vec<i32>> = Vec::new();
        let s_chars: Vec<char> = s.chars().collect(); // Convert string to char vector for easier indexing

        while j < n {
            if s_chars[j as usize] == '0' {
                zero += 1;
            } else {
                one += 1;
            }

            while (j - i + 1) > mid {
                if s_chars[i as usize] == '0' {
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

        let merged = self.merge(intervals, mid);
        merged.len() as i32 <= op
    }

    pub fn get_mini(&self, s: &str, even: char, odd: char) -> i32 {
        let n = s.len();
        let mut op = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for i in 0..n {
            if i % 2 != 0 && s_chars[i] != odd {
                op += 1;
            } else if i % 2 == 0 && s_chars[i] != even {
                op += 1;
            }
        }
        op
    }

    pub fn min_length(&self, s: &str, num_ops: i32) -> i32 {
        let n = s.len() as i32;
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        let mut op = n + 1;
        op = op.min(self.get_mini(s, '0', '1'));
        op = op.min(self.get_mini(s, '1', '0'));
        if op <= num_ops {
            return 1;
        }

        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = self.is_poss(s, num_ops, mid);
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
    let mut solution = Solution {};
    let mut s = String::new();
    let mut num_ops_str = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut num_ops_str).expect("Failed to read line");

    let s = s.trim();
    let num_ops: i32 = num_ops_str.trim().parse().expect("Please type a number!");

    let result = solution.min_length(s, num_ops);

    println!("{}", result);
}