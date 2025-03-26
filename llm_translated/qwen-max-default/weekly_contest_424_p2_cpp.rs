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

    // Check if all elements in nums can be reduced to zero
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
    let mut buffer = String::new();

    // Read the size of the nums array
    stdin.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().expect("Invalid input");
    buffer.clear();

    // Read the nums array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        stdin.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().expect("Invalid input"));
        buffer.clear();
    }

    // Read the number of queries
    stdin.read_line(&mut buffer).unwrap();
    let m: usize = buffer.trim().parse().expect("Invalid input");
    buffer.clear();

    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        stdin.read_line(&mut buffer).unwrap();
        let mut iter = buffer.trim().split_whitespace();
        let a: i32 = iter.next().unwrap().parse().expect("Invalid input");
        let b: i32 = iter.next().unwrap().parse().expect("Invalid input");
        queries.push(vec![a, b]);
        buffer.clear();
    }

    // Call the solution function and print the result
    if is_zero_array(&nums, &queries) {
        writeln!(stdout, "true").unwrap();
    } else {
        writeln!(stdout, "false").unwrap();
    }
}