use std::io;

fn main() {
    // Read the size of the array
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Read the array elements
    let mut nums = vec![0; n];
    for i in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        nums[i] = num_str.trim().parse().expect("Please type a number!");
    }

    // Read the range [l, r]
    let mut lr = String::new();
    io::stdin().read_line(&mut lr).expect("Failed to read line");
    let lr: Vec<usize> = lr
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please type a number!"))
        .collect();
    let (l, r) = (lr[0], lr[1]);

    // Compute the minimum sum subarray
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);
}

/// Finds the minimum sum of any subarray with length between `l` and `r` inclusive.
/// If no such subarray exists, returns -1.
fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let n = nums.len();
    let mut mini = i32::MAX;
    for i in 0..n {
        let mut currsum = 0;
        for j in i..n {
            currsum += nums[j];
            let length = j - i + 1;
            if length >= l && length <= r && currsum > 0 {
                mini = mini.min(currsum);
            }
        }
    }
    if mini == i32::MAX { -1 } else { mini }
}