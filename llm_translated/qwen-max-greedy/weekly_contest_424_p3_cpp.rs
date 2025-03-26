use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    fn min_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];
        
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }
        
        let mut acc = 0;
        let mut cur = 0;
        let mut ans = 0;
        while acc <= 0 && cur < n {
            acc += d[cur];
            cur += 1;
        }
        if cur == n {
            return 0;
        }

        let m = queries.len();
        for i in 0..m {
            let (l, r, val) = (queries[i][0] as usize, queries[i][1] as usize, queries[i][2]);
            d[r + 1] += val;
            d[l] -= val;
            if cur >= l && cur <= r {
                acc -= val;
                while acc <= 0 && cur < n {
                    acc += d[cur];
                    cur += 1;
                }
                if cur == n {
                    return (i + 1) as i32;
                }
            }
        }
        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of the nums array
    stdin.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().unwrap());
    }

    // Read the number of queries
    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let m: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let query: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    let result = Solution::min_zero_array(&nums, &queries);
    writeln!(stdout, "{}", result).unwrap();
}