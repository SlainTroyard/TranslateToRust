use std::io::{self, BufRead};

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    // Create a frequency counter for numbers in range [-1000, 1000]
    let mut set = vec![0; 2001];
    for &num in nums {
        set[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001;
    for &num in nums {
        let curr_sum = total_sum - num;
        if (curr_sum & 1) == 0 {
            // curr_sum is even
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            if half >= -1000 && half <= 1000 {
                if set[(half + 1000) as usize] > threshold {
                    if ans < num {
                        ans = num;
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let nums_size: usize = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number");

    // Input the elements of the array
    let nums: Vec<i32> = lines.next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    // Calculate the largest outlier
    let result = get_largest_outlier(&nums);

    // Output the result
    println!("{}", result);
}