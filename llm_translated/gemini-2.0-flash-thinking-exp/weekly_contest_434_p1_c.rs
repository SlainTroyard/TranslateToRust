use std::io;
use std::str::FromStr;

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    let nums_size = nums.len();
    for i in 0..nums_size - 1 {
        let mut left_sum = 0;
        let mut right_sum = 0;
        for j in 0..=i {
            left_sum += nums[j];
        }
        for j in i + 1..nums_size {
            right_sum += nums[j];
        }
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    // Read the size of the array
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    // Allocate a vector to store the numbers
    let mut nums: Vec<i32> = Vec::with_capacity(n);

    // Read the numbers into the vector
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for number");
        nums.push(num);
    }

    // Call the countPartitions function and print the result
    println!("{}", count_partitions(&nums));
}