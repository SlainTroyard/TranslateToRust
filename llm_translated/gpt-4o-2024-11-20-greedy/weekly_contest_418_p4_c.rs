use std::io::{self, BufRead};
use std::cmp::Ordering;

fn gcd_values(nums: &[usize], queries: &[i64]) -> Vec<usize> {
    let mx = *nums.iter().max().unwrap_or(&0);

    // Count occurrences of each number
    let mut cnt_x = vec![0; mx + 1];
    for &num in nums {
        cnt_x[num] += 1;
    }

    // Calculate gcd pair counts
    let mut cnt_gcd = vec![0i64; mx + 1];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (c as i64 * (c as i64 - 1)) / 2;
    }

    // Prefix sum of gcd counts
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // Answer queries
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
        ans.push(left);
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numsSize and nums array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read queriesSize and queries array
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let queries: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure input sizes match
    assert_eq!(nums.len(), nums_size);
    assert_eq!(queries.len(), queries_size);

    // Compute results
    let results = gcd_values(&nums, &queries);

    // Print results
    for res in results {
        print!("{} ", res);
    }
    println!();
}