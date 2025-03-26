use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];
        
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }
        
        let mut acc = 0;
        let mut cur = -1;
        let mut ans = 0;
        while acc <= 0 && cur < n as i32 - 1 {
            cur += 1;
            acc += d[cur as usize];
        }
        if cur == n as i32 - 1 {
            return 0;
        }

        let m = queries.len();
        for i in 0..m {
            d[(queries[i][1] + 1) as usize] += queries[i][2];
            d[queries[i][0] as usize] -= queries[i][2];
            if cur >= queries[i][0] && cur <= queries[i][1] {
                acc -= queries[i][2];
                while acc <= 0 && cur < n as i32 - 1 {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if cur == n as i32 - 1 {
                    return (i + 1) as i32;
                }
            }
        }
        -1
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the nums array
    let nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(n)
        .collect();

    // Read the number of queries
    let m: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let query: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .take(3)
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    let result = Solution::min_zero_array(&nums, &queries);
    println!("{}", result);

    Ok(())
}