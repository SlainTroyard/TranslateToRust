use std::io;

fn read_all_tokens() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|s| s.to_string()).collect()
}

/// Computes the GCD values for given queries based on the input array.
///
/// # Arguments
/// * `nums` - The array of integers to process.
/// * `queries` - The array of queries to answer.
///
/// # Returns
/// A vector of integers representing the answers to each query.
pub fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap();
    let mx_usize = mx as usize;

    // Frequency array for numbers in nums
    let mut cnt_x = vec![0; mx_usize + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    // Array to store cumulative GCD counts
    let mut cnt_gcd = vec![0i64; mx_usize + 1];

    // Compute GCD counts using inclusion-exclusion principle
    for i in (1..=mx).rev() {
        let i_usize = i as usize;
        let mut c = 0;
        for j in (i..=mx).step_by(i as usize) {
            let j_usize = j as usize;
            c += cnt_x[j_usize];
            cnt_gcd[i_usize] -= cnt_gcd[j_usize];
        }
        cnt_gcd[i_usize] += (c * (c - 1)) as i64 / 2;
    }

    // Compute prefix sums to get cumulative counts up to each GCD value
    for i in 1..=mx {
        let i_usize = i as usize;
        cnt_gcd[i_usize] += cnt_gcd[i_usize - 1];
    }

    // Process each query using binary search
    let mut ans = Vec::with_capacity(queries.len());
    for &q in queries {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid as usize] <= q {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left);
    }
    ans
}

fn main() {
    let tokens = read_all_tokens();

    let mut idx = 0;
    let nums_size = tokens[idx].parse().unwrap();
    idx += 1;
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens[idx].parse().unwrap())
        .collect();
    idx += nums_size as usize;

    let queries_size = tokens[idx].parse().unwrap();
    idx += 1;
    let queries: Vec<i64> = (0..queries_size)
        .map(|_| tokens[idx].parse().unwrap())
        .collect();
    idx += queries_size as usize;

    let ans = gcd_values(&nums, &queries);
    for &a in ans.iter() {
        print!("{} ", a);
    }
    println!();
}