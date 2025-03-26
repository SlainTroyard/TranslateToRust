use std::io::{self, BufRead, Write};

/// Function to count the number of valid selections based on the given algorithm.
fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right = nums.iter().sum();
    let mut ret = 0;

    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() -> io::Result<()> {
    // Read the size of the array from stdin
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().parse().expect("Failed to parse n")
    };

    // Read the array elements from stdin
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        nums.push(input.trim().parse().expect("Failed to parse element"));
    }

    // Calculate the result
    let result = count_valid_selections(&nums);

    // Print the result to stdout
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}