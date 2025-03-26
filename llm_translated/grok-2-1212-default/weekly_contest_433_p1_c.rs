use std::io::{self, BufRead};

fn subarray_sum(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let mut sums = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        sums[i + 1] = nums[i] + sums[i];
        let max_index = (i as i32 - nums[i]).max(0) as usize;
        ans += sums[i + 1] - sums[max_index];
    }
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = subarray_sum(&nums);
    println!("{}", result);

    Ok(())
}