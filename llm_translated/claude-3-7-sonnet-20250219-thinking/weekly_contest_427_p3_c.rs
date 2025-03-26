use std::io::{self, Read};
use std::cmp;

fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    let mut cur = 0i64;
    let mut keep = vec![0i64; n - k + 1];

    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - k + 1] = cur;
            cur -= nums[i - (k - 1)] as i64;
        }
    }

    let mut ans = i64::MIN;

    for i in 0..cmp::min(k, n - k + 1) {
        let mut cur = 0i64;
        let mut max = keep[i];

        let mut l = i;
        while l < n - k + 1 {
            cur += keep[l];

            if cur > max {
                max = cur;
            }
            if cur < 0 {
                cur = 0;
            }
            
            l += k;
        }
        
        ans = cmp::max(ans, max);
    }
    
    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    // Read all input at once
    handle.read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();
    
    // Parse n (size of array)
    let n: usize = tokens.next()
        .expect("Missing input for n")
        .parse()
        .expect("Failed to parse n");
    
    // Parse k
    let k: usize = tokens.next()
        .expect("Missing input for k")
        .parse()
        .expect("Failed to parse k");
    
    // Parse array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens.next()
            .expect("Not enough array elements")
            .parse()
            .expect("Failed to parse array element");
        nums.push(num);
    }

    // Calculate and print the result
    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);

    Ok(())
}