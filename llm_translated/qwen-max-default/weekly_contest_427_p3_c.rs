use std::io;

/// Returns the maximum of two values.
fn max_value(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

/// Computes the maximum subarray sum with a given constraint.
fn max_subarray_sum(nums: &[i32], k: usize) -> i64 {
    let n = nums.len();
    let mut cur = 0;
    let mut keep = vec![0; n - k + 1];

    for (i, &num) in nums.iter().enumerate() {
        cur += num as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans = i64::MIN;
    let mut max = 0;

    for i in 0..k.min(n - k + 1) {
        cur = 0;
        max = keep[i];

        for l in (i..n - k + 1).step_by(k) {
            cur += keep[l];

            if cur > max {
                max = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }
        ans = max_value(ans, max);
    }
    ans
}

fn main() {
    // Read the size of the array and the value of k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please type a number!");

    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Calculate and print the result
    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}