use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_zero_array(nums: &Vec<i32>, queries: &Vec<[i32; 3]>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut d = vec![0; n + 1];
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc: i32 = 0;
        let mut cur: isize = -1;

        while acc <= 0 && (cur as usize) < n {
            cur += 1;
            acc += d[cur as usize];
        }

        if cur as usize == n {
            return 0;
        }

        for (i, query) in queries.iter().enumerate() {
            let l = query[0];
            let r = query[1];
            let val = query[2];

            let r_plus_1 = (r + 1) as usize;
            d[r_plus_1] += val;
            let l_usize = l as usize;
            d[l_usize] -= val;

            let l_isize = l as isize;
            let r_isize = r as isize;

            if cur >= l_isize && cur <= r_isize {
                acc -= val;
                while acc <= 0 && (cur as usize) < n {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if cur as usize == n {
                    return (i as i32) + 1;
                }
            }
        }

        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let nums_line = lines.next().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let m: usize = lines.next().unwrap().parse().unwrap();
    let mut queries = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push([parts[0], parts[1], parts[2]]);
    }

    let solution = Solution;
    let result = solution.min_zero_array(&nums, &queries);
    println!("{}", result);
}