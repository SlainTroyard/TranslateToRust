use std::io;

struct Solution {}

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d: Vec<i32> = vec![0; n + 1];

        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }

        let mut acc: i32 = 0;
        let mut cur: i32 = -1;
        let mut ans: i32 = 0;
        while (acc <= 0) && (cur < n as i32) {
            cur += 1;
            acc += d[cur as usize];
        }
        if cur == n as i32 {
            return 0;
        }

        let m = queries.len();
        for i in 0..m {
            let query = &queries[i];
            d[query[1] as usize + 1] += query[2];
            d[query[0] as usize] -= query[2];
            if (cur >= query[0] as i32) && (cur <= query[1] as i32) {
                acc -= query[2];
                while (acc <= 0) && (cur < n as i32) {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if cur == n as i32 {
                    return i as i32 + 1;
                }
            }
        }
        return -1;
    }
}

fn main() {
    let mut sol = Solution {};

    // Read the size of the nums array
    let mut n_str = String::new();
    io::stdin()
        .read_line(&mut n_str)
        .expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    // Read the nums array
    let mut nums_str = String::new();
    io::stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Read the number of queries
    let mut m_str = String::new();
    io::stdin()
        .read_line(&mut m_str)
        .expect("Failed to read line");
    let m: usize = m_str.trim().parse().expect("Invalid input");

    // Read the queries
    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m {
        let mut query_str = String::new();
        io::stdin()
            .read_line(&mut query_str)
            .expect("Failed to read line");
        let query: Vec<i32> = query_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    println!("{}", sol.min_zero_array(nums, queries));
}