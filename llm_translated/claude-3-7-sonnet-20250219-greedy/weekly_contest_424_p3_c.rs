use std::io::{self, BufRead};

fn min_zero_array(nums: &[i32], queries: &[Vec<i32>]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

    let nums_size = nums.len();
    let mut max = vec![0; nums_size + 1];

    let mut sum = 0;
    let mut k = 0;
    for i in 0..nums_size {
        while sum + max[i] < nums[i] {
            if k == queries.len() {
                return -1;
            }

            let start = queries[k][0] as usize;
            let end = queries[k][1] as usize;
            let increment = queries[k][2];
            k += 1;

            if i > end {
                continue;
            }

            if i > start {
                max[i] += increment;
            } else {
                max[start] += increment;
            }
            max[end + 1] -= increment;
        }
        sum = sum + max[i];
    }

    k
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Reading input for nums
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Reading input for queries
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap().unwrap();
        let query: Vec<i32> = query_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Calling the function
    let result = min_zero_array(&nums, &queries);

    // Output the result
    println!("{}", result);
}