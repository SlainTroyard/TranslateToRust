use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];
        if n > 0 {
            d[0] = nums[0];
        }
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc = 0;
        let mut cur = -1i32;
        while acc <= 0 && cur < n as i32 {
            cur += 1;
            acc += d[cur as usize];
        }

        if cur == n as i32 {
            return 0;
        }

        for (i, query) in queries.iter().enumerate() {
            let l = query[0] as usize;
            let r = query[1] as usize;
            let x = query[2];

            // Update the difference array
            d[r + 1] += x;
            d[l] -= x;

            let current_cur = cur as usize;
            if l <= current_cur && current_cur <= r {
                acc -= x;

                while acc <= 0 && cur < n as i32 {
                    cur += 1;
                    acc += d[cur as usize];
                }

                if cur == n as i32 {
                    return (i + 1) as i32;
                }
            }
        }

        -1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = match tokens.next() {
        Some(val) => val as usize,
        None => {
            println!("-1");
            return;
        }
    };
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(tokens.next().unwrap());
    }

    let m = match tokens.next() {
        Some(val) => val as usize,
        None => {
            println!("-1");
            return;
        }
    };
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let l = tokens.next().unwrap();
        let r = tokens.next().unwrap();
        let x = tokens.next().unwrap();
        queries.push(vec![l, r, x]);
    }

    let result = Solution::min_zero_array(nums, queries);
    println!("{}", result);
}