use std::io;

fn subarray_sum(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut ans = 0;
    let mut sums = vec![0; nums_size + 1];
    
    for i in 0..nums_size {
        sums[i + 1] = nums[i] + sums[i];
        // Use same max logic as original code
        let index = i as i32 - nums[i];
        let prev_index = if index > 0 { index as usize } else { 0 };
        ans += sums[i + 1] - sums[prev_index];
    }
    
    ans
}

fn main() {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Please enter a valid number");
    
    // Allocate and read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Please enter a valid number");
        nums.push(num);
    }
    
    // Calculate and print the result
    println!("{}", subarray_sum(&nums));
}