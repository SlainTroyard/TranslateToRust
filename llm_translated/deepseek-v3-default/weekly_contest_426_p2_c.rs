use std::io::{self, BufRead};

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    let mut set = [0; 2001];
    for &num in nums {
        set[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001;
    for &num in nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 {
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            if (-1000..=1000).contains(&half) {
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

    // Read the size of the array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the largest outlier
    let result = get_largest_outlier(&nums);

    // Output the result
    println!("{}", result);
}