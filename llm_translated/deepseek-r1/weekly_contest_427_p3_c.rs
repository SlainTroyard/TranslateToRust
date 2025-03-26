use std::io::{self, Read};

fn max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let k_usize = k as usize;

    let mut cur = 0i64;
    let mut keep = Vec::new();

    // Calculate sliding window sums of size k
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k_usize - 1 {
            keep.push(cur);
            // Subtract the element leaving the window
            cur -= nums[i - (k_usize - 1)] as i64;
        }
    }

    let mut ans = i64::MIN;

    // Process each possible starting offset modulo k
    for i in 0..(k_usize.min(keep.len())) {
        let mut current = 0i64;
        let mut max_val = keep[i];

        // Kadane's algorithm variant for stepped indices
        for l in (i..keep.len()).step_by(k_usize) {
            current += keep[l];
            max_val = max_val.max(current);
            if current < 0 {
                current = 0;
            }
        }

        ans = ans.max(max_val);
    }

    ans
}

fn main() {
    // Read all input at once and parse as integers
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Parse n (array size) and k (window size)
    let n = tokens.next().expect("Missing array size");
    let k = tokens.next().expect("Missing window size");

    // Collect exactly n elements for the array
    let nums: Vec<i32> = tokens
        .by_ref()
        .take(n as usize)
        .collect();

    // Validate input length matches specification
    if nums.len() != n as usize {
        panic!(
            "Invalid input: expected {} elements, got {}",
            n,
            nums.len()
        );
    }

    // Calculate and print result
    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}