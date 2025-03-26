use std::io;

fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements from stdin
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str)?;
    let n: usize = n_str.trim().parse()?;

    // Initialize the array to hold the numbers
    let mut nums: Vec<i32> = Vec::with_capacity(n);

    // Read the numbers into the array
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str)?;
        let num: i32 = num_str.trim().parse()?;
        nums.push(num);
    }

    // Calculate the result using the function
    let result = count_valid_selections(&nums);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}