use std::io::{self, BufRead, Write};

fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let mut min_sum = None;
    for window_size in l..=r {
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            if i >= window_size {
                sum -= nums[i - window_size];
            }
            if sum > 0 && i >= window_size - 1 {
                min_sum = Some(match min_sum {
                    Some(current_min) => current_min.min(sum),
                    None => sum,
                });
            }
        }
    }
    min_sum.unwrap_or(-1)
}

fn main() -> io::Result<()> {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums_size: usize = input.trim().parse().expect("Invalid input for numsSize");

    // Allocate memory and read the array elements
    let mut nums = vec![0; nums_size];
    for num in nums.iter_mut() {
        input.clear();
        io::stdin().read_line(&mut input)?;
        *num = input.trim().parse().expect("Invalid input for nums");
    }

    // Read the range [l, r]
    input.clear();
    io::stdin().read_line(&mut input)?;
    let range: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input for range"))
        .collect();
    let (l, r) = (range[0], range[1]);

    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}