use std::io;

fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut sums = vec![0; n + 1];
    let mut ans = 0;

    for i in 0..n {
        sums[i + 1] = nums[i] + sums[i];
        let start_index = if i as i32 - nums[i] > 0 {
            (i as i32 - nums[i]) as usize
        } else {
            0
        };
        ans += sums[i + 1] - sums[start_index];
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", subarray_sum(&nums));

    Ok(())
}