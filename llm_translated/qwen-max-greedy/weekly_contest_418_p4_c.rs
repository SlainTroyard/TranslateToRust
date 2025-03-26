use std::io::{self, BufRead, Write};

fn main() {
    // Read the size of nums and nums values from stdin
    let nums_size = read_line_as_usize();
    let mut nums: Vec<usize> = (0..nums_size).map(|_| read_line_as_usize()).collect();

    // Read the size of queries and queries values from stdin
    let queries_size = read_line_as_usize();
    let mut queries: Vec<usize> = (0..queries_size).map(|_| read_line_as_usize()).collect();

    // Call the function to compute the results
    let ans = gcd_values(&mut nums, &mut queries);

    // Print the results
    for &value in &ans {
        print!("{} ", value);
    }
    println!();
}

/// Reads a line from stdin and parses it as usize.
fn read_line_as_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

/// Computes the GCD values based on the given algorithm.
///
/// # Arguments
///
/// * `nums` - A mutable reference to the vector of numbers.
/// * `queries` - A mutable reference to the vector of queries.
///
/// # Returns
///
/// A vector containing the results for each query.
fn gcd_values(nums: &mut [usize], queries: &mut [usize]) -> Vec<usize> {
    let max_num = *nums.iter().max().unwrap_or(&0);

    // Count occurrences of each number
    let mut cnt_x: Vec<usize> = vec![0; max_num + 1];
    for &num in nums {
        cnt_x[num] += 1;
    }

    // Compute GCD counts
    let mut cnt_gcd: Vec<usize> = vec![0; max_num + 1];
    for i in (1..=max_num).rev() {
        let mut c = 0;
        for j in (i..=max_num).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += c * (c - 1) / 2;
    }

    // Accumulate GCD counts
    for i in 1..=max_num {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Process each query
    let mut ans: Vec<usize> = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = max_num;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left);
    }

    ans
}