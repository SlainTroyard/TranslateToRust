fn get_largest_outlier(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let total_sum: i32 = nums.iter().sum();

    let mut set = [0; 2001];
    for &num in nums.iter() {
        set[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001;
    for i in 0..nums_size {
        let curr_sum = total_sum - nums[i];
        if curr_sum % 2 == 0 {
            let half = curr_sum / 2;
            let threshold = if half == nums[i] { 1 } else { 0 };

            if half >= -1000 && half <= 1000 {
                if set[(half + 1000) as usize] > threshold {
                    if ans < nums[i] {
                        ans = nums[i];
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    use std::io;

    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input for numsSize");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums element"))
        .collect();

    // Calculate the largest outlier.
    let result = get_largest_outlier(&nums);

    // Output the result.
    println!("{}", result);
}