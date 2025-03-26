use std::io::{self, BufRead};

fn gcd_values(nums: &[i32], queries: &[i64]) -> Vec<i32> {
    // Find the maximum value in nums
    let mx = *nums.iter().max().unwrap_or(&0);
    
    // Count occurrences of each number in nums
    let mut cnt_x = vec![0; mx as usize + 1];
    for &num in nums {
        cnt_x[num as usize] += 1;
    }
    
    // Calculate the count of pairs with specific GCD values
    let mut cnt_gcd = vec![0i64; mx as usize + 1];
    for i in (1..=mx as usize).rev() {
        let mut c = 0;
        for j in (i..=mx as usize).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c as i64 * (c as i64 - 1)) / 2;
    }
    
    // Calculate prefix sums
    for i in 1..=mx as usize {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }
    
    // Process each query using binary search
    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let mut left = 1;
        let mut right = mx as usize;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read nums size and nums array
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read queries size and queries array
    let queries_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let queries: Vec<i64> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Process the queries
    let result = gcd_values(&nums, &queries);
    
    // Print the results
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}