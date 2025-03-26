use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the nums array
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    // Read the size of the queries array
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the queries array
    let mut queries: Vec<Vec<usize>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap().unwrap();
        let mut query = query_line.split_whitespace();
        let start: usize = query.next().unwrap().parse().unwrap();
        let end: usize = query.next().unwrap().parse().unwrap();
        queries.push(vec![start, end]);
    }

    // Compute the result
    let result = maximum_subarray_xor(nums, queries);

    // Print the result
    for res in result {
        print!("{} ", res);
    }
    println!();
}

fn maximum_subarray_xor(f: Vec<i32>, queries: Vec<Vec<usize>>) -> Vec<i32> {
    let n = f.len();
    let mut mx = vec![vec![0; n]; n];

    // Fill the mx array
    for i in (0..n).rev() {
        mx[i][i] = f[i];
        for j in i + 1..n {
            f[j] ^= f[j - 1];
            mx[i][j] = max(f[j], max(mx[i + 1][j], mx[i][j - 1]));
        }
    }

    // Prepare the answer vector
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let start = q[0];
        let end = q[1];
        ans.push(mx[start][end]);
    }

    ans
}