use std::io;

struct Solution;

impl Solution {
    pub fn merge(&self, intervals: &Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let mut res = vec![intervals[0].clone()];
        for interval in intervals.iter().skip(1) {
            let curr_start = interval[0];
            let curr_end = interval[1];
            let last = res.last().unwrap();
            let prev_start = last[0];
            let prev_end = last[1];
            if curr_start <= prev_end && (curr_start - prev_start + 1) <= len {
                let new_end = std::cmp::max(prev_end, curr_end);
                res.last_mut().unwrap()[1] = new_end;
            } else {
                res.push(interval.clone());
            }
        }
        res
    }

    pub fn is_poss(&self, s: &str, op: i32, mid: i32) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mid_usize = mid as usize;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();
        while j < n {
            let c = s.chars().nth(j).unwrap();
            if c == '0' {
                zero += 1;
            } else {
                one += 1;
            }
            while (j - i + 1) > mid_usize {
                let c_remove = s.chars().nth(i).unwrap();
                if c_remove == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }
            if (j - i + 1) == mid_usize {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }
        let merged = self.merge(&intervals, mid);
        merged.len() as i32 <= op
    }

    pub fn get_mini(&self, s: &str, even_char: char, odd_char: char) -> i32 {
        let mut ops = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c != even_char {
                    ops += 1;
                }
            } else {
                if c != odd_char {
                    ops += 1;
                }
            }
        }
        ops
    }

    pub fn min_length(&self, s: String, num_ops: i32) -> i32 {
        let n = s.len();
        let mut start = 3;
        let mut end = n as i32;
        let mut res = 2;
        let mut op = (n as i32) + 1;
        op = op.min(self.get_mini(&s, '0', '1'));
        op = op.min(self.get_mini(&s, '1', '0'));
        if op <= num_ops {
            return 1;
        }
        while start <= end {
            let mid = start + (end - start) / 2;
            let curr_possible = self.is_poss(&s, num_ops, mid);
            if curr_possible {
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
    let s = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read string");
        input.trim().to_string()
    };

    let num_ops = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read num_ops");
        input.trim().parse::<i32>().expect("Invalid num_ops")
    };

    let solution = Solution {};
    let result = solution.min_length(s, num_ops);
    println!("{}", result);
}