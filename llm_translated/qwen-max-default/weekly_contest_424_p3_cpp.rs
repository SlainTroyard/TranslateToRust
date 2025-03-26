use std::io::{self, BufRead, Write};

fn min_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut d = vec![0; n + 1];
    
    d[0] = nums[0];
    for i in 1..n {
        d[i] = nums[i] - nums[i - 1];
    }
    
    let mut acc = 0;
    let mut cur = -1;
    let mut ans = 0;
    while acc <= 0 && (cur as usize) < n {
        cur += 1;
        acc += d[cur as usize];
    }
    if cur == n as i32 {
        return 0;
    }

    let m = queries.len();
    for i in 0..m {
        d[queries[i][1] as usize + 1] += queries[i][2];
        d[queries[i][0] as usize] -= queries[i][2];
        if (cur as usize) >= queries[i][0] as usize && (cur as usize) <= queries[i][1] as usize {
            acc -= queries[i][2];
            while acc <= 0 && (cur as usize) < n {
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
        stdin.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    // Read the number of queries
    stdin.read_line(&mut buffer).unwrap();
    let m: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        stdin.read_line(&mut buffer).unwrap();
        let query: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
        buffer.clear();
    }

    // Call the solution function and print the result
    let result = min_zero_array(&nums, &queries);
    writeln!(stdout, "{}", result).unwrap();
}