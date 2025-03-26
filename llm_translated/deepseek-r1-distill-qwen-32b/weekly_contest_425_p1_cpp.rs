use std::io;

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid integer");

    // Read the array elements
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid integer"))
        .collect();

    // Read the range [l, r]
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let lr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid integer"))
        .collect();
    let l = lr[0];
    let r = lr[1];

    // Compute the minimum sum subarray
    let mut min_sum = i32::MAX;
    for i in 0..n {
        let mut curr_sum = 0;
        for j in i..n {
            curr_sum += nums[j];
            let length = (j - i + 1) as i32;
            if length >= l && length <= r && curr_sum > 0 {
                if curr_sum < min_sum {
                    min_sum = curr_sum;
                }
            }
        }
    }

    // Output the result
    if min_sum == i32::MAX {
        println!("-1");
    } else {
        println!("{}", min_sum);
    }
}