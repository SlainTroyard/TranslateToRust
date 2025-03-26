use std::io;

fn subarray_sum(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut ans = 0;
    let mut sums: Vec<i32> = vec![0; nums_size + 1]; // Initialize sums vector with size numsSize + 1

    for i in 0..nums_size {
        sums[i + 1] = nums[i] + sums[i]; // Calculate prefix sum
        let start_index = (i as i32 - nums[i] as i32).max(0) as usize; // Calculate max(0, i - nums[i]) to avoid negative index
        ans += sums[i + 1] - sums[start_index]; // Calculate and add the subarray sum
    }
    ans
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n"); // Read integer n from input

    let mut nums: Vec<i32> = Vec::with_capacity(n as usize); // Initialize nums vector with capacity n
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for num"); // Read integer num from input
        nums.push(num); // Add num to nums vector
    }

    let result = subarray_sum(&nums); // Call subarray_sum function
    println!("{}", result); // Print the result
}