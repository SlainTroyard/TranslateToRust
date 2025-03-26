use std::io::{self, BufRead};

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut left: usize = 0;
    let mut right: usize = 1;

    while right < nums.len() {
        if nums[left] < nums[right] {
            ans += (right - left) as i64 * nums[left] as i64;
            left = right;
        }
        right += 1;
    }

    ans + (right - left - 1) as i64 * nums[left] as i64
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .take(nums_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = find_maximum_score(&nums);
    println!("{}", result);

    Ok(())
}