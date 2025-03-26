use std::io::{self, BufRead};

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0i64;
    let mut mx = 0;
    // Iterate through all elements except the last one, maintaining maximum
    for i in 0..nums.len().saturating_sub(1) {
        mx = mx.max(nums[i]);
        ans += mx as i64;
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    // Read vector size from first line
    let nums_size: usize = lines.next()
        .expect("Failed to read nums size")
        .parse()
        .expect("Invalid nums size");
    
    // Read numbers from subsequent input lines, handling multi-line input
    let nums: Vec<i32> = lines
        .flat_map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse().expect("Invalid integer"))
        })
        .take(nums_size)
        .collect();
    
    // Ensure exactly nums_size elements were read (like C++ vector initialization)
    if nums.len() != nums_size {
        panic!("Insufficient numbers provided");
    }
    
    println!("{}", find_maximum_score(&nums));
}