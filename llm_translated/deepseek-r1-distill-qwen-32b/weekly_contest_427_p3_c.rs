use std::io;

fn read_input() -> Result<i32, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    trimmed.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn read_array(n: usize) -> Result<Vec<i32>, std::io::Error> {
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num = read_input()?;
        nums.push(num);
    }
    Ok(nums)
}

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    if k == 0 || n < k {
        return 0;
    }
    let window_count = n - k + 1;
    let mut keep = vec![0; window_count];
    let mut cur = 0;
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            let index = i - (k - 1);
            keep[index] = cur;
            cur -= nums[index] as i64;
        }
    }
    let mut ans = i64::MIN;
    let max_i = std::cmp::min(k, keep.len());
    for i in 0..max_i {
        let mut cur_sum = 0;
        let mut max_sum = keep[i];
        let mut l = i;
        while l < keep.len() {
            cur_sum += keep[l];
            if cur_sum > max_sum {
                max_sum = cur_sum;
            }
            if cur_sum < 0 {
                cur_sum = 0;
            }
            l += k;
        }
        if max_sum > ans {
            ans = max_sum;
        }
    }
    ans
}

fn main() {
    let n = read_input().expect("Failed to read n");
    let k = read_input().expect("Failed to read k");
    let nums = read_array(n as usize).expect("Failed to read array");
    
    let result = max_subarray_sum(&nums, n as usize, k as usize);
    println!("{}", result);
}