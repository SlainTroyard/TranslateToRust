use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the nums array
    let mut nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the number of queries
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the queries
    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(m);
    for _ in 0..m {
        let query: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    let result = min_zero_array(&mut nums, &queries);
    println!("{}", result);
}

fn min_zero_array(nums: &mut Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut d = vec![0; n + 1];

    d[0] = nums[0];
    for i in 1..n {
        d[i] = nums[i] - nums[i - 1];
    }

    let mut acc = 0;
    let mut cur = -1;
    let mut ans = 0;
    while acc <= 0 && cur < n as i32 {
        cur += 1;
        acc += d[cur as usize];
    }
    if cur == n as i32 {
        return 0;
    }

    let m = queries.len();
    for i in 0..m {
        let l = queries[i][0] as usize;
        let r = queries[i][1] as usize;
        let val = queries[i][2];

        d[r + 1] += val;
        d[l] -= val;

        if cur >= l as i32 && cur <= r as i32 {
            acc -= val;
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