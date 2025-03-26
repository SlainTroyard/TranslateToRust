use std::io::{self, BufRead};
use std::cmp::max;

/// Calculates the greatest common divisor of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    return b;
}

/// Calculates the least common multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    return (a / gcd(a, b)) * b;
}

/// Calculates the maximum score based on the given algorithm
fn max_score(nums: &[i32]) -> i64 {
    let nums_size = nums.len();
    if nums_size == 1 {
        return (nums[0] as i64) * (nums[0] as i64);
    }
    
    let mut lcms = vec![0i64; nums_size];
    let mut gcds = vec![0i64; nums_size];
    
    lcms[nums_size - 1] = nums[nums_size - 1] as i64;
    gcds[nums_size - 1] = nums[nums_size - 1] as i64;
    
    for i in (0..nums_size-1).rev() {
        lcms[i] = lcm(nums[i] as i64, lcms[i+1]);
        gcds[i] = gcd(nums[i] as i64, gcds[i+1]);
    }
    
    let mut ans = lcms[0] * gcds[0];
    ans = max(ans, lcms[1] * gcds[1]);
    
    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;
    
    for i in 1..nums_size-1 {
        ans = max(gcd(pregcd, gcds[i+1]) * lcm(prelcm, lcms[i+1]), ans);
        prelcm = lcm(prelcm, nums[i] as i64);
        pregcd = gcd(pregcd, nums[i] as i64);
    }
    
    ans = max(ans, prelcm * pregcd);
    return ans;
}

fn main() {
    // Read numsSize
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let nums_size: usize = lines.next()
        .expect("Failed to read line")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Expected an integer for numsSize");
    
    // Read nums array
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let input = lines.next()
            .expect("Failed to read line")
            .expect("Failed to read line");
        nums.push(input.trim().parse::<i32>().expect("Expected an integer"));
    }
    
    // Calculate and print result
    println!("{}", max_score(&nums));
}