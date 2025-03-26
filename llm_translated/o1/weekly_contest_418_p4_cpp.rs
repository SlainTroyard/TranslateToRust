// Weekly Contest 418 Problem 4 in Rust
//
// This program reproduces the exact logic of the provided C++ solution,
// preserving the same input/output format and algorithm.
//
// Usage (stdin example):
//   n
//   nums[0] nums[1] ... nums[n-1]
//   q
//   queries[0] queries[1] ... queries[q-1]
//
// Output (stdout):
//   ans[0] ans[1] ... ans[q-1]
//   (followed by a newline)

use std::io::{self, Read};

/// Computes the gcd values according to the exact logic in the C++ solution.
fn gcd_values(nums: &Vec<i32>, queries: &Vec<i64>) -> Vec<i32> {
    // Find maximum element in nums
    let mx = *nums.iter().max().unwrap() as usize;

    // Count how many times each number appears
    let mut cnt_x = vec![0; mx + 1];
    for &x in nums {
        cnt_x[x as usize] += 1;
    }

    // cnt_gcd[i] will eventually hold the number of pairs whose gcd is exactly i.
    // After computing exact gcd counts, we will convert it to a prefix sum array.
    let mut cnt_gcd = vec![0i64; mx + 1];

    // Calculate the number of pairs with gcd exactly i, by inclusion-exclusion:
    // We iterate from mx down to 1, subtract counts of multiples, then add pairs.
    for i in (1..=mx).rev() {
        let mut c = 0i64;
        let mut j = i;
        while j <= mx {
            c += cnt_x[j] as i64;
            cnt_gcd[i] -= cnt_gcd[j];
            j += i;
        }
        cnt_gcd[i] += c * (c - 1) / 2; // number of pairs from c elements
    }

    // Convert cnt_gcd into a prefix sum array. After this, cnt_gcd[i]
    // contains the number of pairs whose gcd is <= i.
    for i in 1..=mx {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    // For each query, we use an upper_bound search over cnt_gcd
    // to find the smallest index i where cnt_gcd[i] > query.
    // Because we're directly reproducing upper_bound, that means:
    // ans[i] = index in [0..mx+1].
    let mut ans = Vec::with_capacity(queries.len());
    for &q in queries {
        ans.push(upper_bound(&cnt_gcd, q) as i32);
    }

    ans
}

/// A direct replication of C++ std::upper_bound for i64 slices.
/// Returns the index of the first element in arr that is strictly greater than x.
fn upper_bound(arr: &[i64], x: i64) -> usize {
    let mut low = 0usize;
    let mut high = arr.len();
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] <= x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn main() -> io::Result<()> {
    // Read all input from stdin at once (mimics the behavior of C++ cin which can read
    // mixed whitespace-delimited integers from multiple lines).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();

    // Read n
    let n: usize = tokens
        .next()
        .unwrap()
        .parse()
        .expect("Failed to parse n");

    // Read nums
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i32 = tokens
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse nums element");
        nums.push(x);
    }

    // Read q
    let q: usize = tokens
        .next()
        .unwrap()
        .parse()
        .expect("Failed to parse q");

    // Read queries
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let x: i64 = tokens
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse query");
        queries.push(x);
    }

    // Compute the answers
    let ans = gcd_values(&nums, &queries);

    // Print results, space-separated, then a newline
    for x in ans {
        print!("{} ", x);
    }
    println!();

    Ok(())
}