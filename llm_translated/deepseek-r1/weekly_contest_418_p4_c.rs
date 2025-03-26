use std::io::{self, Read};

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    let mx = *nums.iter().max().unwrap_or(&0) as usize;

    // Count occurrences of each number
    let mut cnt_x = vec![0; mx + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }

    // Calculate cnt_gcd using inclusion-exclusion principle
    let mut cnt_gcd = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        let mut j = i;
        while j <= mx {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        cnt_gcd[i] += (c as i64) * (c as i64 - 1) / 2;
    }

    // Compute prefix sums for cnt_gcd
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Process each query with binary search
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx;
        while left < right {
            let mid = (left + right) / 2;
            if cnt_gcd[mid] <= query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(left as i32);
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse nums
    let nums_size = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid nums size");
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens.next().and_then(|s| s.parse().ok()).expect("Invalid num"))
        .collect();

    // Parse queries
    let queries_size = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid queries size");
    let queries: Vec<i64> = (0..queries_size)
        .map(|_| tokens.next().and_then(|s| s.parse().ok()).expect("Invalid query"))
        .collect();

    let ans = gcd_values(&nums, &queries);

    // Print results with trailing space and newline
    for num in ans {
        print!("{} ", num);
    }
    println!();
}