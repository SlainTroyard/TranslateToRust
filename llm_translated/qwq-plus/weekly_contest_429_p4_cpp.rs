use std::io;

struct Solution;

impl Solution {
    pub fn merge(intervals: &Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let mut res = vec![intervals[0].clone()];
        for interval in intervals.iter().skip(1) {
            let curr_start = interval[0];
            let curr_end = interval[1];
            let prev = res.last().unwrap();
            let prev_start = prev[0];
            let prev_end = prev[1];
            if curr_start <= prev_end && (curr_start - prev_start + 1) <= len {
                let new_end = std::cmp::max(prev_end, curr_end);
                if let Some(last) = res.last_mut() {
                    last[1] = new_end;
                }
            } else {
                res.push(interval.clone());
            }
        }
        res
    }

    pub fn is_poss(&self, s: &str, op: i32, mid: i32) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();
        while j < n {
            let c = chars[j];
            if c == '0' {
                zero += 1;
            } else {
                one += 1;
            }
            while (j - i + 1) as i32 > mid {
                let c_i = chars[i];
                if c_i == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }
            if (j - i + 1) == mid as usize {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }
        let merged = Solution::merge(&intervals, mid);
        merged.len() <= op as usize
    }

    pub fn get_mini(&self, s: &str, even: char, odd: char) -> i32 {
        let mut op = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c != even {
                    op += 1;
                }
            } else {
                if c != odd {
                    op += 1;
                }
            }
        }
        op
    }

    pub fn min_length(&self, s: &str, num_ops: i32) -> i32 {
        let n = s.len();
        let mut res = 2;
        let mut op = (n as i32) + 1;
        op = op.min(self.get_mini(s, '0', '1'));
        op = op.min(self.get_mini(s, '1', '0'));
        if op <= num_ops {
            return 1;
        }
        let mut start = 3;
        let mut end = n as i32;
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let num_ops: i32 = parts[1].parse().expect("Invalid number of operations");
    let solution = Solution {};
    let result = solution.min_length(&s, num_ops);
    println!("{}", result);
}