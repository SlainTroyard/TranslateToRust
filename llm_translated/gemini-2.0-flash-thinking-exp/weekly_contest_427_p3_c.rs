use std::io;
use std::cmp;

fn max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let k_usize = k as usize;
    if n < k_usize {
        return i64::MIN; // Or handle as per problem spec if k > n
    }

    let mut cur: i64 = 0;
    let mut keep: Vec<i64> = Vec::new();

    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k_usize - 1 {
            keep.push(cur);
            cur -= nums[i - k_usize + 1] as i64;
        }
    }

    let mut ans: i64 = i64::MIN;
    let mut max: i64;

    for i in 0..cmp::min(k_usize, keep.len()) {
        cur = 0;
        max = keep[i];

        let mut l = i;
        while l < keep.len() {
            cur += keep[l];

            max = cmp::max(max, cur);
            if cur < 0 {
                cur = 0;
            }
            l += k_usize;
        }
        ans = cmp::max(ans, max);
    }
    return ans;
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let k: i32 = k_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}