use std::io;

/// Counts the number of valid selections in the array based on the given conditions.
fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right = nums.iter().sum::<i32>();
    let mut result = 0;

    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                result += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                result += 1;
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Read the first line for the number of elements
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Invalid input for n");

    // Read the next line for the array elements
    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer in input"))
        .collect();

    // Ensure that the number of elements matches n
    assert_eq!(nums.len(), n, "Number of elements does not match n");

    let result = count_valid_selections(&nums);
    println!("{}", result);

    Ok(())
}