use std::io;

fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let mut min_sum: Option<i32> = None;

    for window_size in l..=r {
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];

            if i >= window_size {
                sum -= nums[i - window_size];
            }

            if sum > 0 && i >= window_size - 1 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }

    min_sum.unwrap_or(-1)
}

fn main() {
    let mut input = String::new();

    // Input the size of the array
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Invalid input");

    // Allocate memory for the array
    let mut nums = Vec::with_capacity(nums_size);

    // Input the array elements
    for _ in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    // Input the range [l, r]
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut lr = input.trim().split_whitespace();
    let l: usize = lr.next().expect("Invalid input").parse().expect("Invalid input");
    let r: usize = lr.next().expect("Invalid input").parse().expect("Invalid input");

    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);
}