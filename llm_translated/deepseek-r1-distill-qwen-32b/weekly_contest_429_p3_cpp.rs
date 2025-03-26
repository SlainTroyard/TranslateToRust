use std::cmp;

struct Solution;

impl Solution {
    fn merge(intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        let mut res = vec![intervals[0].clone()];
        for i in 1..intervals.len() {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];
            let last = res.last().unwrap();
            if curr_start <= last[1] && (curr_start - last[0] + 1) <= len {
                res.last_mut().unwrap()[1] = cmp::max(last[1], curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    fn is_poss(&self, s: String, op: i32, mid: i32) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
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

            while (j - i + 1) as i32 > mid {
                let c_i = s.chars().nth(i).unwrap();
                if c_i == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            if (j - i + 1) as i32 == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }

        let merged = Self::merge(intervals, mid);
        merged.len() as i32 <= op
    }

    fn get_mini(&self, s: String, even: char, odd: char) -> i32 {
        let mut op = 0;
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            if i % 2 == 1 {
                if c != odd {
                    op += 1;
                }
            } else {
                if c != even {
                    op += 1;
                }
            }
        }
        op
    }

    fn min_length(&self, s: String, num_ops: i32) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let mut res = 2;
        let mut op = (n + 1) as i32;
        op = cmp::min(op, self.get_mini(s.clone(), '0', '1'));
        op = cmp::min(op, self.get_mini(s.clone(), '1', '0'));
        if op <= num_ops {
            return 1;
        }

        let mut start = 3;
        let mut end = n as i32;
        while start <= end {
            let mid = start + (end - start) / 2;
            if self.is_poss(s.clone(), num_ops, mid) {
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
    let mut solution = Solution;
    let s = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };
    let num_ops = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    };

    let result = solution.min_length(s, num_ops);
    println!("{}", result);
}