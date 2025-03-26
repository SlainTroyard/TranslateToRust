use std::io::{self, BufRead};

fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right = nums.iter().sum::<i32>();
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the second line to get the array elements
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the number of elements matches the input
    assert_eq!(nums.len(), n);

    let result = count_valid_selections(&nums);
    println!("{}", result);

    Ok(())
}