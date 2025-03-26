use std::io::{self, BufRead, Write};

fn is_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> bool {
    let n = nums.len();
    let mut v = vec![0; n + 1];
    
    // Apply the range updates
    for q in queries {
        v[q[0] as usize] += 1;
        if (q[1] + 1) as usize < n {
            v[(q[1] + 1) as usize] -= 1;
        }
    }
    
    // Compute the prefix sums
    for i in 1..n {
        v[i] += v[i - 1];
    }
    
    // Check if the final array can be made zero
    for i in 0..n {
        if nums[i] - v[i] > 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read the size of the nums array
    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    // Read the nums array
    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the number of queries
    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let m: usize = buffer.trim().parse().unwrap();

    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        let query: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Call the solution function and print the result
    if is_zero_array(&nums, &queries) {
        writeln!(stdout, "true").unwrap();
    } else {
        writeln!(stdout, "false").unwrap();
    }
}