use std::cmp::max;
use std::io;
use std::io::Read;
use std::i64;

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur: i64 = 0;
    let mut keep: Vec<i64> = Vec::with_capacity(n - k + 1);

    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep.push(cur);
            cur -= nums[i - k + 1] as i64;
        }
    }

    let mut ans: i64 = i64::MIN;

    for i in 0..k.min(n - k + 1) {
        cur = 0;
        let mut max_val = keep[i];

        let mut l = i;
        while l < n - k + 1 {
            cur += keep[l];

            if cur > max_val {
                max_val = cur;
            }
            if cur < 0 {
                cur = 0;
            }
            l += k;
        }
        ans = max(ans, max_val);
    }
    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let k: usize = lines.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);

    Ok(())
}