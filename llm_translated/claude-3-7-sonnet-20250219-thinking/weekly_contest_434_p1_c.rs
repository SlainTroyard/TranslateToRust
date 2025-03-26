use std::io::{self, BufRead};

/// Count partitions where the difference between left and right sums is even
fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        let mut left_sum = 0;
        let mut right_sum = 0;
        
        // Calculate sum of left partition
        for j in 0..=i {
            left_sum += nums[j];
        }
        
        // Calculate sum of right partition
        for j in i + 1..nums.len() {
            right_sum += nums[j];
        }
        
        // Check if difference is even (divisible by 2)
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Expected an integer for array size");
    
    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse()
            .expect("Expected an integer");
        nums.push(num);
    }
    
    // Calculate and print the result
    println!("{}", count_partitions(&nums));
}