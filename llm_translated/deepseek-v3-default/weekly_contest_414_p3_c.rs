use std::io::{self, BufRead};

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0i64;
    let mut l = 0usize;
    let mut r = 1usize;
    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }
    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure the number of elements matches the size
    assert_eq!(nums.len(), nums_size);
    
    // Compute and print the result
    println!("{}", find_maximum_score(&nums));
}