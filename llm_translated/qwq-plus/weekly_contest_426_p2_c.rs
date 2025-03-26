use std::io;

/// Finds the largest outlier in the array according to the problem's logic.
///
/// The outlier is defined as an element such that when removed, the remaining sum can be split into two equal parts.
/// The function uses a frequency array to track element occurrences and checks each element's validity.
fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    // Frequency array to track occurrences of each number in the range [-1000, 1000].
    let mut freq = vec![0; 2001];
    for &num in nums {
        let idx = (num + 1000) as usize;
        freq[idx] += 1;
    }

    let mut ans = -1001; // Initialize with the minimum possible value

    for &num in nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 != 0 {
            continue; // Skip if the remaining sum is odd
        }

        let half = curr_sum / 2;
        let threshold = if half == num { 1 } else { 0 };

        // Check if half is within the valid range for the frequency array
        if half < -1000 || half > 1000 {
            continue;
        }

        let idx = (half + 1000) as usize;
        if freq[idx] > threshold {
            if num > ans {
                ans = num;
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input numbers
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let nums_size = iter.next().unwrap() as usize;
    let nums: Vec<i32> = iter.take(nums_size).collect();

    // Calculate and print the result
    let result = get_largest_outlier(&nums);
    println!("{}", result);
}